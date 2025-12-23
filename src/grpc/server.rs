// path: src/grpc/server.rs
use crate::clients::whisper::WhisperClient;
use sentiric_contracts::sentiric::stt::v1::stt_gateway_service_server::SttGatewayService;
use sentiric_contracts::sentiric::stt::v1::{
    TranscribeStreamRequest, TranscribeStreamResponse,
    WhisperTranscribeStreamRequest
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};
use tracing::{info, warn, error, instrument};
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
        
        // 1. TRACE ID EXTRACT
        let trace_id = request.metadata()
            .get("x-trace-id")
            .and_then(|m| m.to_str().ok())
            .map(|s| s.to_string());

        info!("🎧 STT Stream Connection Established. TraceID: {}", trace_id.as_deref().unwrap_or("none"));
        
        let inbound_stream = request.into_inner();

        // 2. INPUT MAPPING (Gateway -> Whisper)
        // İstemciden gelen stream'i Whisper formatına çevir
        let outbound_stream = inbound_stream.filter_map(|res| {
            match res {
                Ok(req) => Some(WhisperTranscribeStreamRequest {
                    audio_chunk: req.audio_chunk,
                }),
                Err(e) => {
                    warn!("Inbound stream packet error: {}", e);
                    None
                }
            }
        });

        // 3. UPSTREAM CALL (Whisper'a Bağlan)
        let mut whisper_response_stream = self.whisper_client.transcribe_stream(outbound_stream, trace_id.clone()).await
            .map_err(|e| {
                error!("Failed to connect to Whisper Engine: {}", e);
                Status::unavailable("Expert STT Engine Unavailable")
            })?;

        // 4. OUTPUT MAPPING (Whisper -> Gateway -> Client)
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
                            warn!("Client disconnected, stopping stream.");
                            break; 
                        }
                    }
                    Err(e) => {
                        error!("Upstream stream error: {}", e);
                        let _ = tx.send(Err(Status::internal("Error received from STT Engine"))).await;
                        break;
                    }
                }
            }
            info!("STT Stream Completed. TraceID: {}", trace_id.as_deref().unwrap_or("none"));
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}