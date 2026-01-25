use crate::config::AppConfig;
use crate::clients::whisper::WhisperClient;
use crate::grpc::server::SttGateway;
use crate::tls::load_server_tls_config;
use crate::metrics::start_metrics_server; // EKLENDİ
use sentiric_contracts::sentiric::stt::v1::stt_gateway_service_server::SttGatewayServiceServer;
use tonic::transport::Server;
use std::net::SocketAddr;
use tracing::{info, warn};
use anyhow::Result;
use std::sync::Arc;

pub struct App;

impl App {
    pub async fn run() -> Result<()> {
        let config = Arc::new(AppConfig::load()?);
        tracing_subscriber::fmt().with_env_filter(&config.rust_log).init();
        info!("🚀 STT Gateway Service v{} starting...", config.service_version);

        let whisper_client = WhisperClient::connect(&config).await?;
        
        // Metrics Server
        let metrics_addr: SocketAddr = format!("{}:{}", config.host, config.http_port).parse()?;
        start_metrics_server(metrics_addr, whisper_client.clone());

        let addr: SocketAddr = format!("{}:{}", config.host, config.grpc_port).parse()?;
        let gateway_service = SttGateway::new(whisper_client);
        
        let mut builder = Server::builder();

        if !config.stt_gateway_service_cert_path.is_empty() && !config.grpc_tls_ca_path.is_empty() {
             match load_server_tls_config(&config).await {
                Ok(tls) => {
                    builder = builder.tls_config(tls)?;
                    info!("🎧 gRPC Server listening on {} (mTLS Enabled)", addr);
                },
                Err(e) => {
                     warn!("⚠️ TLS Load Failed: {}. Falling back to INSECURE.", e);
                     info!("🎧 gRPC Server listening on {} (INSECURE)", addr);
                }
             }
        } else {
             warn!("⚠️ TLS paths empty. Starting in INSECURE mode.");
             info!("🎧 gRPC Server listening on {} (INSECURE)", addr);
        }

        builder
            .add_service(SttGatewayServiceServer::new(gateway_service))
            .serve(addr)
            .await?;

        Ok(())
    }
}