use crate::config::AppConfig;
use crate::error::GatewayError;
use sentiric_contracts::sentiric::stt::v1::{
    stt_gateway_service_server::SttGatewayService,
    stt_whisper_service_client::SttWhisperServiceClient,
    TranscribeRequest, TranscribeResponse, TranscribeStreamRequest, TranscribeStreamResponse,
    WhisperTranscribeStreamRequest,
};
use std::sync::Arc;
use tokio_stream::{Stream, StreamExt};
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
        Err(Status::unimplemented(
            "Tekil transcribe henüz desteklenmiyor.",
        ))
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
                GatewayError::UpstreamConnectionFailed {
                    url: upstream_url,
                    source: e,
                }
            })?;

        let inbound_stream = request.into_inner();

        // GÜNCELLEME: Gelen akıştaki Result'ları işleyerek uzman motorun beklediği tipe dönüştür.
        // `filter_map` ile sadece başarılı (Ok) mesajları alıp dönüştürüyoruz.
        // Hatalı (Err) bir mesaj gelirse loglayıp akışı o noktada sonlandırıyoruz.
        let whisper_request_stream = inbound_stream.filter_map(|result| async move {
            match result {
                Ok(req) => Some(WhisperTranscribeStreamRequest {
                    audio_chunk: req.audio_chunk,
                }),
                Err(status) => {
                    warn!("İstemci akışında bir hata oluştu, akış sonlandırılıyor: {}", status);
                    None
                }
            }
        });

        info!("Akış, uzman STT motoruna yönlendiriliyor...");
        let upstream_response = client
            .whisper_transcribe_stream(whisper_request_stream)
            .await
            .map_err(GatewayError::UpstreamStreamError)?;
            
        // GÜNCELLEME: Uzman motordan gelen akışı istemcinin beklediği tipe dönüştür.
        // Bu kısım, tipler farklı olduğu için hala gereklidir ve doğru çalışır.
        let upstream_stream = upstream_response.into_inner();
        let transformed_outbound_stream = upstream_stream.map(|item| {
            item.map(|resp| TranscribeStreamResponse {
                partial_transcription: resp.transcription,
                is_final: resp.is_final,
            })
        });

        info!("Akış başarıyla yönlendirildi ve istemciye geri iletiliyor.");
        Ok(Response::new(Box::pin(transformed_outbound_stream)))
    }
}