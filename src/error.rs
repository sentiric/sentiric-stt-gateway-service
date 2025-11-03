use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum GatewayError {
    #[error("Uzman STT servisine ulaşılamıyor: {0}")]
    UpstreamUnavailable(String),
    #[error("Uzman STT servisinden geçersiz yanıt: {0}")]
    UpstreamInvalidResponse(String),
}

impl From<GatewayError> for Status {
    fn from(err: GatewayError) -> Self {
        match err {
            GatewayError::UpstreamUnavailable(msg) => Status::unavailable(msg),
            _ => Status::internal(err.to_string()),
        }
    }
}