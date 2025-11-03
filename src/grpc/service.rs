use crate::config::AppConfig;
use crate::error::GatewayError;
use sentiric_contracts::sentiric::stt::v1::{
    stt_gateway_service_server::SttGatewayService,
    stt_whisper_service_client::SttWhisperServiceClient,
    TranscribeRequest, TranscribeResponse, TranscribeStreamRequest, TranscribeStreamResponse,
};
use std::sync::Arc;
use tokio_stream::Stream;
use tonic::{transport::Channel, Request, Response, Status, Streaming};
use tracing::{error, info, instrument, warn};

#[derive(Debug)]
pub struct MySttGateway {
    config: Arc<AppConfig>,
}

impl MySttGateway {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self { config }
    }
}

#[tonic::async_trait]
impl SttGatewayService for MySttGateway {
    #[instrument(skip(self, _request), fields(service = "stt-gateway-service"))]
    async fn transcribe(
        &self,
        _request: Request<TranscribeRequest>,
    ) -> Result<Response<TranscribeResponse>, Status> {
        warn!("Tekil transcribe metodu çağrıldı ancak henüz implemente edilmedi.");
        Err(Status::unimplemented("Tekil transcribe henüz desteklenmiyor."))
    }

    type TranscribeStreamStream =
        std::pin::Pin<Box<dyn Stream<Item = Result<TranscribeStreamResponse, Status>> + Send>>;

    #[instrument(skip_all, fields(service = "stt-gateway-service"))]
    async fn transcribe_stream(
        &self,
        request: Request<Streaming<TranscribeStreamRequest>>,
    ) -> Result<Response<Self::TranscribeStreamStream>, Status> {
        info!("Yeni bir TranscribeStream isteği alındı.");
        let upstream_url = self.config.stt_whisper_service_target_grpc_url.clone();

        let mut client = SttWhisperServiceClient::<Channel>::connect(upstream_url.clone())
            .await
            .map_err(|e| {
                error!("Uzman STT motoruna bağlanılamadı: {}", e);
                GatewayError::UpstreamConnectionFailed { url: upstream_url, source: e }
            })?;

        info!("Akış, uzman STT motoruna yönlendiriliyor...");
        let upstream_response = client
            .whisper_transcribe_stream(request)
            .await
            .map_err(GatewayError::UpstreamStreamError)?;
            
        let upstream_stream = upstream_response.into_inner();
        info!("Akış başarıyla yönlendirildi.");
        Ok(Response::new(Box::pin(upstream_stream)))
    }
}