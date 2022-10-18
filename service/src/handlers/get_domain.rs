use signaling_proto::message::{GetDomainRequest, GetDomainResponse};
use tonic::Status;

#[tracing::instrument]
pub async fn handle_get_domain(req: GetDomainRequest) -> Result<GetDomainResponse, Status> {
    match std::env::var("DOMAIN") {
        Ok(domain_var) => Ok(GetDomainResponse { domain: domain_var }),
        Err(err) => {
            tracing::error!(?err, "get env 'DOMAIN' failed");
            Err(Status::internal("read 'DOMAIN' failed"))
        }
    }
}
