// Dosya: src/clients/whisper.rs
use crate::config::AppConfig;
use crate::tls::load_client_tls_config;
use sentiric_contracts::sentiric::stt::v1::stt_whisper_service_client::SttWhisperServiceClient;
use sentiric_contracts::sentiric::stt::v1::{WhisperTranscribeStreamRequest, WhisperTranscribeStreamResponse};
use tonic::transport::{Channel, Endpoint};
use tonic::Request;
use futures::Stream;
use std::sync::Arc;
use tracing::{info, error};
use tonic::metadata::MetadataValue;
use std::str::FromStr;

#[derive(Clone)]
pub struct WhisperClient {
    client: SttWhisperServiceClient<Channel>,
}

impl WhisperClient {
    pub async fn connect(config: &Arc<AppConfig>) -> anyhow::Result<Self> {
        let url = config.stt_whisper_service_grpc_url.clone();
        
        //[ARCH-COMPLIANCE] constraints.yaml'ın gerektirdiği şekilde tüm gRPC bağlantılarında mTLS zorunlu kılındı. HTTP fallback kaldırıldı.
        if url.starts_with("http://") {
            anyhow::bail!("Insecure connection attempt to {}. mTLS (https://) is REQUIRED by architecture constraints.", url);
        }

        info!(
            event = "UPSTREAM_CONNECTING",
            url = %url,
            "🔐 Connecting to Whisper Service (mTLS)"
        );
        let tls_config = load_client_tls_config(config).await?;
        let channel = Endpoint::from_shared(url)?
            .tls_config(tls_config)?
            .connect()
            .await?;

        Ok(Self {
            client: SttWhisperServiceClient::new(channel),
        })
    }

    pub async fn transcribe_stream(
        &self,
        request_stream: impl Stream<Item = WhisperTranscribeStreamRequest> + Send + 'static,
        trace_id: Option<String>,
        span_id: Option<String>,
        tenant_id: Option<String>,
    ) -> Result<tonic::Streaming<WhisperTranscribeStreamResponse>, tonic::Status> {
        let mut client = self.client.clone();
        let mut request = Request::new(request_stream);

        // [ARCH-COMPLIANCE] Context Propagation - x-trace-id, x-span-id, x-tenant-id
        if let Some(ref tid) = trace_id {
            if let Ok(meta_val) = MetadataValue::from_str(tid) {
                request.metadata_mut().insert("x-trace-id", meta_val);
            }
        }
        
        if let Some(ref sid) = span_id {
            if let Ok(meta_val) = MetadataValue::from_str(sid) {
                request.metadata_mut().insert("x-span-id", meta_val);
            }
        }

        if let Some(ref ten) = tenant_id {
            if let Ok(meta_val) = MetadataValue::from_str(ten) {
                request.metadata_mut().insert("x-tenant-id", meta_val);
            }
        }
        
        match client.whisper_transcribe_stream(request).await {
            Ok(response) => Ok(response.into_inner()),
            Err(e) => {
                error!(
                    event = "UPSTREAM_CALL_FAILED",
                    trace_id = ?trace_id,
                    span_id = ?span_id,
                    tenant_id = ?tenant_id,
                    error = %e,
                    "❌ Whisper Engine gRPC call failed"
                );
                Err(e)
            }
        }
    }

    pub fn is_ready(&self) -> bool {
        true
    }
}