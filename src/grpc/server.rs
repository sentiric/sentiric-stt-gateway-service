use crate::clients::whisper::WhisperClient;
use sentiric_contracts::sentiric::stt::v1::stt_gateway_service_server::SttGatewayService;
use sentiric_contracts::sentiric::stt::v1::{
    TranscribeStreamRequest, TranscribeStreamResponse,
    WhisperTranscribeStreamRequest
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};
use tracing::{info, error, instrument};
use futures::StreamExt;

pub struct SttGateway {
    whisper_client: WhisperClient,
}

impl SttGateway {
    pub fn new(whisper_client: WhisperClient) -> Self {
        Self { whisper_client }
    }
}

#[tonic::async_trait]
impl SttGatewayService for SttGateway {
    type TranscribeStreamStream = ReceiverStream<Result<TranscribeStreamResponse, Status>>;

    async fn transcribe(
        &self,
        _request: Request<sentiric_contracts::sentiric::stt::v1::TranscribeRequest>,
    ) -> Result<Response<sentiric_contracts::sentiric::stt::v1::TranscribeResponse>, Status> {
        Err(Status::unimplemented("Use TranscribeStream for real-time STT"))
    }

    #[instrument(skip(self, request))]
    async fn transcribe_stream(
        &self,
        request: Request<Streaming<TranscribeStreamRequest>>,
    ) -> Result<Response<Self::TranscribeStreamStream>, Status> {
        info!("STT Stream Connection Established.");
        
        let inbound_stream = request.into_inner();

        // 1. INPUT MAPPING (Gateway -> Whisper)
        // İstemciden gelen `TranscribeStreamRequest` paketlerini,
        // Whisper motorunun beklediği `WhisperTranscribeStreamRequest` paketlerine çeviriyoruz.
        // `filter_map` hatalı paketleri sessizce yutmak yerine loglayıp devam edebilir.
        let outbound_stream = inbound_stream.filter_map(|res| {
            match res {
                Ok(req) => Some(WhisperTranscribeStreamRequest {
                    audio_chunk: req.audio_chunk,
                    // Eğer gateway request içinde config varsa buraya eklenebilir
                    // Şimdilik sadece raw audio chunk iletiyoruz.
                }),
                Err(e) => {
                    error!("Inbound stream error: {}", e);
                    None
                }
            }
        });

        // 2. UPSTREAM CALL
        let mut whisper_response_stream = self.whisper_client.transcribe_stream(outbound_stream).await
            .map_err(|e| Status::unavailable(format!("Whisper Engine unavailable: {}", e)))?;

        // 3. OUTPUT MAPPING (Whisper -> Gateway -> Client)
        let (tx, rx) = tokio::sync::mpsc::channel(128);

        tokio::spawn(async move {
            while let Some(result) = whisper_response_stream.next().await {
                match result {
                    Ok(w_resp) => {
                        let g_resp = TranscribeStreamResponse {
                            partial_transcription: w_resp.transcription,
                            is_final: w_resp.is_final,
                        };
                        if tx.send(Ok(g_resp)).await.is_err() {
                            break; // Client koptu
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(Err(Status::internal(format!("Upstream error: {}", e)))).await;
                        break;
                    }
                }
            }
            info!("STT Stream Completed.");
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}