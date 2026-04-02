// File: sentiric-stt-gateway-service/src/grpc/server.rs
use crate::clients::whisper::WhisperClient;
use futures::StreamExt;
use sentiric_contracts::sentiric::stt::v1::stt_gateway_service_server::SttGatewayService;
use sentiric_contracts::sentiric::stt::v1::{
    TranscribeStreamRequest, TranscribeStreamResponse, WhisperTranscribeStreamRequest,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};
use tracing::{error, info, instrument, warn};

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
        Err(Status::unimplemented(
            "Use TranscribeStream for real-time STT",
        ))
    }

    #[instrument(skip(self, request))]
    async fn transcribe_stream(
        &self,
        request: Request<Streaming<TranscribeStreamRequest>>,
    ) -> Result<Response<Self::TranscribeStreamStream>, Status> {
        let trace_id_opt = request
            .metadata()
            .get("x-trace-id")
            .and_then(|m| m.to_str().ok())
            .map(|s| s.to_string());
        let span_id_opt = request
            .metadata()
            .get("x-span-id")
            .and_then(|m| m.to_str().ok())
            .map(|s| s.to_string());
        let tenant_id_opt = request
            .metadata()
            .get("x-tenant-id")
            .and_then(|m| m.to_str().ok())
            .map(|s| s.to_string());

        // Display için string'e açma
        let trace_id = trace_id_opt.as_deref().unwrap_or("unknown");
        let span_id = span_id_opt.as_deref().unwrap_or("unknown");
        let tenant_id = tenant_id_opt.as_deref().unwrap_or("unknown");

        if tenant_id == "unknown" || tenant_id.trim().is_empty() {
            error!(event = "MISSING_TENANT_ID", trace_id = %trace_id, span_id = %span_id, "Tenant ID is missing in request. Request rejected.");
            return Err(Status::invalid_argument(
                "tenant_id is strictly required for isolation",
            ));
        }

        // ESKİ: trace_id = ?trace_id
        // YENİ: trace_id = %trace_id (SUTS v4.0 Uyumlu)
        info!(event = "STT_STREAM_CONNECTION_ESTABLISHED", trace_id = %trace_id, span_id = %span_id, tenant_id = %tenant_id, "🎧 STT Stream Connection Established.");

        // İlerideki loglar için kopyalar
        let t_id_clone = trace_id.to_string();
        let s_id_clone = span_id.to_string();
        let ten_id_clone = tenant_id.to_string();

        let inbound_stream = request.into_inner();
        let outbound_stream = inbound_stream.filter_map(move |res| {
            let result = match res {
                Ok(req) => Some(WhisperTranscribeStreamRequest { audio_chunk: req.audio_chunk }),
                Err(e) => {
                    warn!(event = "INBOUND_STREAM_ERROR", trace_id = %t_id_clone, span_id = %s_id_clone, tenant_id = %ten_id_clone, error = %e, "Inbound stream packet error");
                    None
                }
            };
            std::future::ready(result)
        });

        // Whisper client çağrısına Option'ları geçir (client kendi içinde hallediyor)
        let mut whisper_response_stream = self.whisper_client.transcribe_stream(
            outbound_stream, trace_id_opt.clone(), span_id_opt.clone(), tenant_id_opt.clone()
        ).await.map_err(|e| {
            error!(event = "STT_ENGINE_CONNECTION_FAILED", trace_id = %trace_id, span_id = %span_id, tenant_id = %tenant_id, error = %e, "Failed to connect to Whisper Engine");
            Status::unavailable("Expert STT Engine Unavailable")
        })?;

        let (tx, rx) = tokio::sync::mpsc::channel(128);

        let t_id_final = trace_id.to_string();
        let s_id_final = span_id.to_string();
        let ten_id_final = tenant_id.to_string();

        tokio::spawn(async move {
            while let Some(result) = whisper_response_stream.next().await {
                match result {
                    Ok(w_resp) => {
                        // [CRITICAL FIX E0063]: Yeni eklenen Affective alanları eşleştirildi.
                        let g_resp = TranscribeStreamResponse {
                            partial_transcription: w_resp.transcription,
                            is_final: w_resp.is_final,
                            gender_proxy: w_resp.gender_proxy,
                            emotion_proxy: w_resp.emotion_proxy,
                            arousal: w_resp.arousal,
                            valence: w_resp.valence,
                        };
                        if tx.send(Ok(g_resp)).await.is_err() {
                            warn!(event = "CLIENT_DISCONNECTED", trace_id = ?t_id_final, span_id = ?s_id_final, tenant_id = ?ten_id_final, "Client disconnected, stopping stream.");
                            break;
                        }
                    }

                    Err(e) => {
                        error!(event = "STT_STREAM_ERROR", trace_id = %t_id_final, span_id = %s_id_final, tenant_id = %ten_id_final, error = %e, "Upstream stream error received from STT Engine.");
                        let _ = tx
                            .send(Err(Status::internal("Error received from STT Engine")))
                            .await;
                        break;
                    }
                }
            }
            info!(event = "STT_STREAM_COMPLETED", trace_id = %t_id_final, span_id = %s_id_final, tenant_id = %ten_id_final, "STT Stream Completed.");
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
