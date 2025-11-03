// main.rs - DOĞRU HALİ

// Projemizin kendi kütüphanesini ve modüllerini `use` ile çağırıyoruz
use sentiric_stt_gateway_service::{
    config::AppConfig,
    grpc::service::MySttGateway,
};

use anyhow::Result;
use axum::{routing::get, Router};
use sentiric_contracts::sentiric::stt::v1::stt_gateway_service_server::SttGatewayServiceServer;
use std::net::SocketAddr;
use tonic::transport::Server;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter, Registry};
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::load()?;
    
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.log_level));
    let subscriber = Registry::default().with(filter);
    if config.env == "development" {
        subscriber.with(fmt::layer()).init();
    } else {
        subscriber.with(fmt::layer().json()).init();
    }

    info!(version = %config.service_version, "STT Gateway Servisi başlatılıyor...");

    let grpc_task = tokio::spawn(run_grpc_server(config.grpc_listen_addr));
    let http_task = tokio::spawn(run_http_server(config.http_listen_addr));

    tokio::try_join!(grpc_task, http_task)?;
    Ok(())
}

async fn run_grpc_server(addr: SocketAddr) -> Result<()> {
    let stt_gateway = MySttGateway::default();
    let grpc_server = SttGatewayServiceServer::new(stt_gateway);
    info!("gRPC sunucusu dinlemede: {}", addr);
    Server::builder().add_service(grpc_server).serve(addr).await?;
    Ok(())
}

async fn run_http_server(addr: SocketAddr) -> Result<()> {
    let http_app = Router::new().route("/health", get(|| async { "ok" }));
    info!("HTTP sağlık kontrolü dinlemede: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, http_app).await?;
    Ok(())
}