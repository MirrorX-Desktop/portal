use crate::handlers::{Client, CLIENTS};
use once_cell::sync::Lazy;
use signaling_proto::message::{PublishMessage, SubscribeRequest};
use tokio::sync::mpsc::{channel, Receiver};
use tokio::sync::Mutex;
use tonic::Status;

static SUBSCRIBE_INSERT_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

#[tracing::instrument]
pub async fn handle_subscribe(
    req: SubscribeRequest,
) -> Result<Receiver<Result<PublishMessage, Status>>, Status> {
    if CLIENTS.contains_key(&req.device_id) {
        return Err(Status::already_exists(
            "device already connected to signaling server",
        ));
    }

    let (tx, rx) = channel(16);
    let client = Client::new(req.device_id, tx);

    let _ = SUBSCRIBE_INSERT_MUTEX.lock().await;

    if CLIENTS.contains_key(&req.device_id) {
        return Err(Status::already_exists(
            "device already connected to signaling server",
        ));
    }

    CLIENTS.insert(req.device_id, client);

    Ok(rx)
}
