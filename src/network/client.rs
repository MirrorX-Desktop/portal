use super::{
    handler::{handle_handshake, handle_heartbeat},
    message::{SignalingMessage, SignalingMessageError, SignalingMessagePacketType},
};
use crate::{component::serializer::BINCODE_SERIALIZER, network::message::SignalingMessagePacket};
use anyhow::bail;
use bincode::Options;
use bytes::Bytes;
use dashmap::DashMap;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use log::{error, info};
use once_cell::sync::Lazy;
use scopeguard::defer;
use std::{
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{
    net::TcpStream,
    sync::mpsc::{Receiver, Sender},
    time::timeout,
};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

pub static CLIENTS: Lazy<DashMap<String, Arc<Client>>> = Lazy::new(|| DashMap::new());
static CLIENTS_INSERT_MUTEX: Lazy<tokio::sync::Mutex<()>> =
    Lazy::new(|| tokio::sync::Mutex::new(()));

pub struct Client {
    #[allow(dead_code)]
    device_id: String,
    packet_tx: Sender<Vec<u8>>,
    atomic_call_id: AtomicU8,
    call_reply_tx_map: DashMap<u8, Sender<SignalingMessage>>,
}

impl Client {
    pub async fn serve(stream: TcpStream) -> anyhow::Result<()> {
        if let Err(err) = stream.set_nodelay(true) {
            bail!("set nodelay for stream failed ({:?})", err);
        }

        let mut framed_stream = LengthDelimitedCodec::builder()
            .little_endian()
            .max_frame_length(8 * 1024)
            .new_framed(stream);

        // handle handshake message

        let packet_bytes = framed_stream
            .next()
            .await
            .ok_or(anyhow::anyhow!("recevied EOF while waiting for handshake"))?
            .map_err(|err| anyhow::anyhow!("handshake message is invalid ({})", err))?;

        let packet = BINCODE_SERIALIZER
            .deserialize::<SignalingMessagePacket>(&packet_bytes)
            .map_err(|err| {
                anyhow::anyhow!("deserialize handshake request message failed ({:?})", err)
            })?;

        if packet.typ != SignalingMessagePacketType::Request {
            bail!(
                "handshake message type is {:?}, except {:?}",
                packet.typ,
                SignalingMessagePacketType::Request
            );
        }

        let handshake_request = match packet.message {
            SignalingMessage::HandshakeRequest(req) => req,
            _ => bail!("serve: handshake message is not SignalingMessage::HandshakeRequest"),
        };

        let mut device_id = None;
        let packet = match handle_handshake(handshake_request).await {
            Ok(resp) => {
                device_id = Some(resp.device_id.clone());
                SignalingMessagePacket {
                    direction: None,
                    typ: SignalingMessagePacketType::Response,
                    call_id: packet.call_id,
                    message: SignalingMessage::HandshakeResponse(resp),
                }
            }
            Err(_) => SignalingMessagePacket {
                direction: None,
                typ: SignalingMessagePacketType::Response,
                call_id: packet.call_id,
                message: SignalingMessage::Error(SignalingMessageError::Internal),
            },
        };

        let packet_bytes = match BINCODE_SERIALIZER.serialize::<SignalingMessagePacket>(&packet) {
            Ok(packet) => packet,
            Err(err) => {
                bail!("serialize handshake response message failed ({:?})", err);
            }
        };

        if let Err(err) = framed_stream.send(Bytes::from(packet_bytes)).await {
            bail!("send handshake response message failed ({:?})", err);
        }

        // create client

        let device_id = match device_id {
            Some(id) => id,
            None => return Err(anyhow::anyhow!("handshake handler returns error")),
        };

        let (packet_tx, packet_rx) = tokio::sync::mpsc::channel(16);
        let client = Arc::new(Client {
            device_id: device_id.clone(),
            packet_tx,
            atomic_call_id: AtomicU8::new(0),
            call_reply_tx_map: DashMap::new(),
        });

        CLIENTS_INSERT_MUTEX.lock().await;
        if CLIENTS.contains_key(&device_id) {
            bail!("client with device id '{}' already exists", device_id);
        } else {
            CLIENTS.insert(device_id, client.clone());
        }

        let (sink, stream) = framed_stream.split();
        serve_stream(client, stream);
        serve_sink(packet_rx, sink);

        return Ok(());
    }

    pub async fn call(
        &self,
        direction: (String, String),
        message: SignalingMessage,
        duration: Duration,
    ) -> Result<SignalingMessage, SignalingMessageError> {
        let call_id = self.atomic_call_id.fetch_add(1, Ordering::SeqCst);

        let packet = SignalingMessagePacket {
            direction: Some(direction),
            typ: SignalingMessagePacketType::Request,
            call_id,
            message,
        };

        let mut rx = self.register_call(call_id);
        defer! {
            self.remove_call(call_id);
        }

        timeout(duration, async move {
            if let Err(err) = self.send(packet).await {
                error!("call: {}", err);
                return Err(SignalingMessageError::Internal);
            }

            rx.recv().await.ok_or(SignalingMessageError::Internal)
        })
        .await
        .map_err(|_| SignalingMessageError::Internal)?
    }

    async fn reply(
        &self,
        direction: Option<(String, String)>,
        call_id: u8,
        message: SignalingMessage,
    ) -> Result<(), SignalingMessageError> {
        let packet = SignalingMessagePacket {
            direction,
            typ: SignalingMessagePacketType::Response,
            call_id,
            message,
        };

        self.send(packet).await.map_err(|err| {
            error!("reply: {:?}", err);
            SignalingMessageError::Internal
        })
    }

    async fn send(&self, packet: SignalingMessagePacket) -> anyhow::Result<()> {
        let buffer = BINCODE_SERIALIZER
            .serialize(&packet)
            .map_err(|err| anyhow::anyhow!("client send message failed ({:?})", err))?;

        self.packet_tx
            .try_send(buffer)
            .map_err(|err| anyhow::anyhow!("client send message failed ({:?})", err))
    }

    fn set_call_reply(&self, call_id: u8, message: SignalingMessage) {
        self.remove_call(call_id).map(|tx| {
            if let Err(err) = tx.try_send(message) {
                error!("set_call_reply: set reply failed ({:?})", err)
            }
        });
    }

    fn register_call(&self, call_id: u8) -> Receiver<SignalingMessage> {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        self.call_reply_tx_map.insert(call_id, tx);
        rx
    }

    fn remove_call(&self, call_id: u8) -> Option<Sender<SignalingMessage>> {
        self.call_reply_tx_map.remove(&call_id).map(|entry| entry.1)
    }
}

fn serve_stream(
    client: Arc<Client>,
    mut stream: SplitStream<Framed<TcpStream, LengthDelimitedCodec>>,
) {
    tokio::spawn(async move {
        loop {
            let packet_bytes = match stream.next().await {
                Some(res) => match res {
                    Ok(packet_bytes) => packet_bytes,
                    Err(err) => {
                        error!("serve_stream: read failed ({:?})", err);
                        break;
                    }
                },
                None => {
                    info!("serve_stream: stream closed, going to exit");
                    break;
                }
            };

            let packet =
                match BINCODE_SERIALIZER.deserialize::<SignalingMessagePacket>(&packet_bytes) {
                    Ok(packet) => packet,
                    Err(err) => {
                        error!("serve_stream: deserialize packet failed ({})", err);
                        break;
                    }
                };

            let client = client.clone();
            tokio::spawn(async move {
                handle_message(client, packet).await;
            });
        }

        info!("serve stream read loop exit");
    });
}

fn serve_sink(
    mut packet_rx: Receiver<Vec<u8>>,
    mut sink: SplitSink<Framed<TcpStream, LengthDelimitedCodec>, Bytes>,
) {
    tokio::spawn(async move {
        loop {
            let buffer = match packet_rx.recv().await {
                Some(buffer) => buffer,
                None => {
                    info!("signaling_serve_sink: packet_rx all sender has dropped, going to exit");
                    break;
                }
            };

            // trace!(buffer = ?format!("{:02X?}", buffer), "signaling_serve_sink: send");

            if let Err(err) = sink.send(Bytes::from(buffer)).await {
                error!(
                    "signaling_serve_sink: send failed, going to exit ({:?})",
                    err
                );
                break;
            }
        }

        info!("signaling_serve_sink: exit");
    });
}

async fn handle_message(client: Arc<Client>, packet: SignalingMessagePacket) {
    match packet.typ {
        SignalingMessagePacketType::Request => {
            let direction = packet.direction;

            let response_message = if let Some((from_device_id, to_device_id)) = direction.clone() {
                match CLIENTS.get(&to_device_id) {
                    Some(remote_client) => remote_client
                        .call(
                            (from_device_id, to_device_id),
                            packet.message,
                            Duration::from_secs(50),
                        )
                        .await
                        .unwrap_or_else(|err| SignalingMessage::Error(err)),
                    None => SignalingMessage::Error(SignalingMessageError::RemoteDeviceOffline),
                }
            } else {
                match packet.message {
                    SignalingMessage::HeartBeatRequest(req) => handle_heartbeat(req)
                        .await
                        .map(|resp| SignalingMessage::HeartBeatResponse(resp))
                        .unwrap_or_else(|err| SignalingMessage::Error(err)),
                    _ => SignalingMessage::Error(SignalingMessageError::Invalid),
                }
            };

            let direction = if let Some((from_device_id, to_device_id)) = direction {
                Some((to_device_id, from_device_id))
            } else {
                None
            };

            let _ = client
                .reply(direction, packet.call_id, response_message)
                .await;
        }
        SignalingMessagePacketType::Response => {
            client.set_call_reply(packet.call_id, packet.message)
        }
    }
}
