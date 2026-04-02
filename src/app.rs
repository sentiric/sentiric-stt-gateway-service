// Dosya: src/app.rs
use crate::clients::whisper::WhisperClient;
use crate::config::AppConfig;
use crate::grpc::server::SttGateway;
use crate::logger::SutsV4Formatter; // [YENİ]
use crate::metrics::start_metrics_server;
use crate::tls::load_server_tls_config;
use anyhow::Result;
use sentiric_contracts::sentiric::stt::v1::stt_gateway_service_server::SttGatewayServiceServer;
use std::net::SocketAddr;
use std::sync::Arc;
use tonic::transport::Server;
use tracing::{error, info};

pub struct App;

impl App {
    pub async fn run() -> Result<()> {
        let config = Arc::new(AppConfig::load()?);

        //[ARCH-COMPLIANCE] Özel SUTS v4.0 Formatter ile tracing_subscriber ayağa kaldırılır
        let formatter = SutsV4Formatter {
            service_name: "stt-gateway-service".to_string(),
            service_version: config.service_version.clone(),
            service_env: config.env.clone(),
        };

        tracing_subscriber::fmt()
            .with_env_filter(&config.rust_log)
            .event_format(formatter)
            .init();

        info!(
            event = "SERVICE_START",
            "🚀 STT Gateway Service starting..."
        );

        let whisper_client = WhisperClient::connect(&config).await?;

        // Metrics Server
        let metrics_addr: SocketAddr = format!("{}:{}", config.host, config.http_port).parse()?;
        start_metrics_server(metrics_addr, whisper_client.clone());

        let addr: SocketAddr = format!("{}:{}", config.host, config.grpc_port).parse()?;
        let gateway_service = SttGateway::new(whisper_client);

        let mut builder = Server::builder();

        // [ARCH-COMPLIANCE] constraints.yaml'ın gerektirdiği şekilde gRPC iletişiminde mTLS zorunlu kılındı.
        if config.stt_gateway_service_cert_path.is_empty() || config.grpc_tls_ca_path.is_empty() {
            anyhow::bail!("TLS configuration paths cannot be empty. mTLS is REQUIRED by architecture constraints.");
        }

        let tls = load_server_tls_config(&config).await.map_err(|e| {
            anyhow::anyhow!(
                "⚠️ TLS Load Failed: {}. mTLS is strictly required. Shutting down.",
                e
            )
        })?;

        builder = builder.tls_config(tls)?;

        info!(
            event = "GRPC_SERVER_READY",
            address = %addr,
            "🎧 gRPC Server listening (mTLS Enabled)"
        );

        if let Err(e) = builder
            .add_service(SttGatewayServiceServer::new(gateway_service))
            .serve(addr)
            .await
        {
            error!(event = "GRPC_SERVER_CRASH", error = %e, "gRPC Server stopped unexpectedly.");
            return Err(e.into());
        }

        Ok(())
    }
}
