use crate::config::AppConfig;
use crate::tls::load_client_tls_config;
use sentiric_contracts::sentiric::stt::v1::stt_whisper_service_client::SttWhisperServiceClient;
use sentiric_contracts::sentiric::stt::v1::{WhisperTranscribeStreamRequest, WhisperTranscribeStreamResponse};
use tonic::transport::{Channel, Endpoint};
use tonic::Request;
use futures::Stream;
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
pub struct WhisperClient {
    client: SttWhisperServiceClient<Channel>,
}

impl WhisperClient {
    pub async fn connect(config: &Arc<AppConfig>) -> anyhow::Result<Self> {
        let url = config.stt_whisper_url.clone();
        info!("Connecting to Whisper Service at: {}", url);

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
        // Gelen herhangi bir stream (Async Iterator) kaynağını kabul eder
        request_stream: impl Stream<Item = WhisperTranscribeStreamRequest> + Send + 'static,
    ) -> Result<tonic::Streaming<WhisperTranscribeStreamResponse>, tonic::Status> {
        let mut client = self.client.clone();
        
        // Stream'i gRPC isteğine sar
        let request = Request::new(request_stream);
        
        // Upstream çağrısı
        let response = client.whisper_transcribe_stream(request).await?;
        
        Ok(response.into_inner())
    }
}