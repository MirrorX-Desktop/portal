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
) -> Result<(i64, Receiver<Result<PublishMessage, Status>>), Status> {
    if CLIENTS.contains_key(&req.device_id) {
        return Err(Status::already_exists(
            "device already connected to signaling server",
        ));
    }

    // check if device info is valid
    let entity = crate::db::device::query_device_by_id(req.device_id)
        .await
        .map_err(|err| {
            tracing::error!(?err, "query_device_by_id");
            Status::internal("internal database query failed")
        })?
        .ok_or_else(|| Status::not_found("device id not register"))?;

    if entity.finger_print != req.device_finger_print
        || entity.expire <= chrono::Utc::now().timestamp()
    {
        tracing::info!(
            "fingerprint match {:?}",
            entity.finger_print == req.device_finger_print
        );

        tracing::info!(
            "expire match {:?}",
            entity.expire <= chrono::Utc::now().timestamp()
        );

        return Err(Status::invalid_argument(
            "device subscribe params is invalid",
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
    tracing::info!(?req.device_id, "subscribe");

    Ok((req.device_id, rx))
}
