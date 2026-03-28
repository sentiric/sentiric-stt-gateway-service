// Dosya: src/grpc/server.rs
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
        
        // 1. TRACE, SPAN, TENANT ID EXTRACT (SUTS v4.0 Compliance)
        let trace_id = request.metadata()
            .get("x-trace-id")
            .and_then(|m| m.to_str().ok())
            .map(|s| s.to_string());
            
        let span_id = request.metadata()
            .get("x-span-id")
            .and_then(|m| m.to_str().ok())
            .map(|s| s.to_string());
            
        let tenant_id = request.metadata()
            .get("x-tenant-id")
            .and_then(|m| m.to_str().ok())
            .map(|s| s.to_string());

        // [ARCH-COMPLIANCE] Strict Tenant Isolation Fail-Fast
        if tenant_id.is_none() || tenant_id.as_deref().unwrap_or("").is_empty() {
            error!(
                event = "MISSING_TENANT_ID",
                trace_id = ?trace_id,
                span_id = ?span_id,
                "Tenant ID is missing in request. Request rejected."
            );
            return Err(Status::invalid_argument("tenant_id is strictly required for isolation"));
        }

        info!(
            event = "STT_STREAM_CONNECTION_ESTABLISHED",
            trace_id = ?trace_id,
            span_id = ?span_id,
            tenant_id = ?tenant_id,
            "🎧 STT Stream Connection Established."
        );
        
        let inbound_stream = request.into_inner();

        // 2. INPUT MAPPING (Gateway -> Whisper)
        let t_id_clone = trace_id.clone();
        let s_id_clone = span_id.clone();
        let ten_id_clone = tenant_id.clone();

        let outbound_stream = inbound_stream.filter_map(move |res| {
            let result = match res {
                Ok(req) => Some(WhisperTranscribeStreamRequest {
                    audio_chunk: req.audio_chunk,
                }),
                Err(e) => {
                    warn!(
                        event = "INBOUND_STREAM_ERROR",
                        trace_id = ?t_id_clone,
                        span_id = ?s_id_clone,
                        tenant_id = ?ten_id_clone,
                        error = %e,
                        "Inbound stream packet error"
                    );
                    None
                }
            };
            std::future::ready(result)
        });

        // 3. UPSTREAM CALL (Whisper'a Bağlan)
        let mut whisper_response_stream = self.whisper_client.transcribe_stream(
            outbound_stream, 
            trace_id.clone(),
            span_id.clone(),
            tenant_id.clone()
        ).await
        .map_err(|e| {
            error!(
                event = "STT_ENGINE_CONNECTION_FAILED",
                trace_id = ?trace_id,
                span_id = ?span_id,
                tenant_id = ?tenant_id,
                error = %e,
                "Failed to connect to Whisper Engine"
            );
            Status::unavailable("Expert STT Engine Unavailable")
        })?;

        // 4. OUTPUT MAPPING (Whisper -> Gateway -> Client)
        let (tx, rx) = tokio::sync::mpsc::channel(128);
        
        let t_id_final = trace_id.clone();
        let s_id_final = span_id.clone();
        let ten_id_final = tenant_id.clone();

        tokio::spawn(async move {
            while let Some(result) = whisper_response_stream.next().await {
                match result {
                    Ok(w_resp) => {
                        let g_resp = TranscribeStreamResponse {
                            partial_transcription: w_resp.transcription,
                            is_final: w_resp.is_final,
                        };
                        if tx.send(Ok(g_resp)).await.is_err() {
                            warn!(
                                event = "CLIENT_DISCONNECTED",
                                trace_id = ?t_id_final,
                                span_id = ?s_id_final,
                                tenant_id = ?ten_id_final,
                                "Client disconnected, stopping stream."
                            );
                            break; 
                        }
                    }
                    Err(e) => {
                        // [ARCH-COMPLIANCE] Surgical Folding of internal engine errors
                        error!(
                            event = "STT_STREAM_ERROR",
                            trace_id = ?t_id_final,
                            span_id = ?s_id_final,
                            tenant_id = ?ten_id_final,
                            error = %e,
                            "Upstream stream error received from STT Engine."
                        );
                        let _ = tx.send(Err(Status::internal("Error received from STT Engine"))).await;
                        break;
                    }
                }
            }
            info!(
                event = "STT_STREAM_COMPLETED",
                trace_id = ?t_id_final,
                span_id = ?s_id_final,
                tenant_id = ?ten_id_final,
                "STT Stream Completed."
            );
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}