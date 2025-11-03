use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum GatewayError {
    #[error("Uzman STT motoruna ({url}) bağlanılamadı: {source}")]
    UpstreamConnectionFailed {
        url: String,
        #[source]
        source: tonic::transport::Error,
    },

    #[error("Uzman STT motoru akışında hata: {0}")]
    UpstreamStreamError(#[from] Status),
}

impl From<GatewayError> for Status {
    fn from(err: GatewayError) -> Self {
        match err {
            GatewayError::UpstreamConnectionFailed { url, .. } => {
                Status::unavailable(format!("Bağımlı servis olan STT motoruna ({}) ulaşılamıyor.", url))
            }
            GatewayError::UpstreamStreamError(status) => status,
        }
    }
}