use crate::message::*;
use bincode::{
    config::{LittleEndian, VarintEncoding, WithOtherEndian, WithOtherIntEncoding},
    DefaultOptions, Options,
};
use bytes::Bytes;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use once_cell::sync::Lazy;
use std::{io, net::SocketAddr, ops::Deref, sync::Arc, time::Duration};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[allow(clippy::type_complexity)]
pub static SUBSCRIBERS: Lazy<
    // use Mutex to make sure that passive device only serve at most one visit call simultaneously
    moka::future::Cache<i64, (Arc<Mutex<()>>, tokio::sync::mpsc::Sender<ServerMessage>)>,
> = Lazy::new(|| {
    moka::future::CacheBuilder::new(256)
        .time_to_idle(Duration::from_secs(5 * 60))
        .build()
});

static SUBSCRIBERS_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

#[allow(clippy::type_complexity)]
pub static CALLS: Lazy<
    moka::future::Cache<(i64, i64), tokio::sync::mpsc::Sender<Result<Vec<u8>, VisitFailureReason>>>,
> = Lazy::new(|| {
    moka::future::CacheBuilder::new(256)
        .time_to_live(Duration::from_secs(80))
        .build()
});

static BINARY_SERIALIZER: Lazy<
    WithOtherIntEncoding<WithOtherEndian<DefaultOptions, LittleEndian>, VarintEncoding>,
> = Lazy::new(|| {
    bincode::DefaultOptions::new()
        .with_little_endian()
        .with_varint_encoding()
});

pub async fn serve_subscriber_server(addr: SocketAddr) -> io::Result<()> {
    let listener = tokio::net::TcpListener::bind(addr).await?;

    loop {
        let (stream, addr) = match listener.accept().await {
            Ok(v) => v,
            Err(err) => {
                tracing::warn!(?err, "accept incoming connection failed");
                return Err(err);
            }
        };

        tracing::info!(?addr, "accept incoming connection");
        if let Err(err) = serve_connection(stream, addr).await {
            tracing::info!(?err, ?addr, "serve connection failed");
        }
    }
}

async fn serve_connection(stream: TcpStream, addr: SocketAddr) -> anyhow::Result<()> {
    stream.set_nodelay(true)?;
    let mut framed_stream = Framed::new(
        stream,
        LengthDelimitedCodec::builder()
            .length_field_length(2)
            .little_endian()
            .new_codec(),
    );

    let subscription = match tokio::time::timeout(Duration::from_secs(30), framed_stream.next())
        .await
    {
        Ok(frame) => match frame {
            Some(frame) => match frame {
                Ok(buffer) => match BINARY_SERIALIZER.deserialize::<Subscription>(buffer.deref()) {
                    Ok(subscription) => subscription,
                    Err(err) => {
                        anyhow::bail!(err)
                    }
                },
                Err(err) => {
                    anyhow::bail!(err);
                }
            },
            None => {
                anyhow::bail!("connection disconnected");
            }
        },
        Err(_) => {
            anyhow::bail!("wait handshake timeout");
        }
    };

    // todo: check subscription device_id and finger_print

    let (sink, stream) = framed_stream.split();

    let _ = SUBSCRIBERS_MUTEX.lock().await;

    if !SUBSCRIBERS.contains_key(&subscription.device_id) {
        tracing::info!(
            ?addr,
            device_id = subscription.device_id,
            "bind connection with device_id"
        );
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        SUBSCRIBERS
            .insert(subscription.device_id, (Arc::new(Mutex::new(())), tx))
            .await;
        tokio::spawn(serve_sink(subscription.device_id, sink, rx));
        tokio::spawn(serve_stream(subscription.device_id, stream));
    }

    Ok(())
}

async fn serve_sink(
    device_id: i64,
    mut sink: SplitSink<Framed<TcpStream, LengthDelimitedCodec>, Bytes>,
    mut rx: tokio::sync::mpsc::Receiver<ServerMessage>,
) {
    loop {
        let Some(server_message) = rx.recv().await else {
            SUBSCRIBERS.invalidate(&device_id).await;
            tracing::info!(?device_id, "subscriber tx closed, drop sink");
            return;
        };

        if let Ok(buffer) = BINARY_SERIALIZER.serialize(&server_message) {
            if let Err(err) = sink.send(Bytes::from(buffer)).await {
                SUBSCRIBERS.invalidate(&device_id).await;
                tracing::error!(?device_id, ?err, "send to subscriber failed, drop sink");
                return;
            }
        }
    }
}

async fn serve_stream(
    device_id: i64,
    mut stream: SplitStream<Framed<TcpStream, LengthDelimitedCodec>>,
) {
    loop {
        let Some(buffer) = stream.next().await else {
            SUBSCRIBERS.invalidate(&device_id).await;
            tracing::info!(?device_id, "receive from subscriber failed, drop stream");
            return;
        };

        let Ok(buffer) = buffer else {
            SUBSCRIBERS.invalidate(&device_id).await;
            tracing::info!(?device_id, "subscriber received buffer framed failed, drop stream");
            return;
        };

        let Ok(client_message) =
            BINARY_SERIALIZER.deserialize::<'_, ClientMessage>(buffer.deref()) else {
                continue;
            };

        match client_message {
            ClientMessage::Ping(value) => {
                // get will extend the client's life time
                if let Some((_, subscribe_tx)) = SUBSCRIBERS.get(&device_id) {
                    if subscribe_tx.send(ServerMessage::Pong(value)).await.is_err() {
                        tracing::info!(?device_id, "subscriber tx closed, drop stream");
                    }
                }
            }
            ClientMessage::VisitResponse {
                active_device_id,
                passive_device_id,
                result,
            } => {
                let key = (active_device_id, passive_device_id);
                if let Some(tx) = CALLS.get(&key) {
                    CALLS.invalidate(&key).await;
                    let _ = tx.send(result).await;
                }
            }
        }
    }
}
