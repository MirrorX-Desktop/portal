use signaling_proto::message::{HeartbeatRequest, HeartbeatResponse};
use tonic::Status;

pub async fn handle_heartbeat(req: HeartbeatRequest) -> Result<HeartbeatResponse, Status> {
    todo!()
}
