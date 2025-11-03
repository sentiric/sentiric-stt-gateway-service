// main.rs - TAM VE DOĞRU HALİ

use sentiric_stt_gateway_service::{
    config::AppConfig,
    grpc::service::MySttGateway,
};
use anyhow::Result;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use sentiric_contracts::sentiric::stt::v1::stt_gateway_service_server::SttGatewayServiceServer;
use serde_json::json;
use std::sync::Arc;
use tonic::transport::{Endpoint, Server};
use tracing::{error, info, instrument};
use tracing_subscriber::{fmt, EnvFilter, Registry};
use tracing_subscriber::prelude::*;

#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    let config = Arc::new(AppConfig::load()?);

    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(&config.log_level))
        .unwrap();
    let subscriber = Registry::default().with(filter);

    if config.env.to_lowercase() == "development" {
        subscriber.with(fmt::layer()).init();
    } else {
        subscriber.with(fmt::layer().json()).init();
    }

    info!(
        version = %config.service_version,
        env = %config.env,
        "STT Gateway Servisi başlatılıyor..."
    );

    let grpc_task = tokio::spawn(run_grpc_server(config.clone()));
    let http_task = tokio::spawn(run_http_server(config.clone()));

    // GÜNCELLEME: try_join sonucunu match ile doğru şekilde ele alıyoruz.
    match tokio::try_join!(grpc_task, http_task) {
        Ok((grpc_res, http_res)) => {
            grpc_res?; // gRPC görevinden gelen Result'ı kontrol et
            http_res?; // HTTP görevinden gelen Result'ı kontrol et
        }
        Err(join_err) => {
            // Bir görevin paniklemesi durumunda hatayı döndür
            return Err(anyhow::Error::from(join_err));
        }
    }
    
    Ok(())
}

async fn run_grpc_server(config: Arc<AppConfig>) -> Result<()> {
    let addr = config.grpc_listen_addr;
    let stt_gateway = MySttGateway::new(config);
    let grpc_server = SttGatewayServiceServer::new(stt_gateway);
    info!("gRPC sunucusu dinlemede: {}", addr);
    Server::builder().add_service(grpc_server).serve(addr).await?;
    Ok(())
}

async fn run_http_server(config: Arc<AppConfig>) -> Result<()> {
    let addr = config.http_listen_addr;
    let http_app = Router::new()
        .route("/health", get(health_check))
        .with_state(config);
    info!("HTTP sağlık kontrolü dinlemede: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, http_app).await?;
    Ok(())
}

#[instrument(skip(config), fields(service = "stt-gateway-service"))]
async fn health_check(
    axum::extract::State(config): axum::extract::State<Arc<AppConfig>>,
) -> impl IntoResponse {
    let whisper_url = config.stt_whisper_service_target_grpc_url.clone();
    
    info!("Sağlık kontrolü: Uzman STT motoru kontrol ediliyor...");

    // GÜNCELLEME: `?` operatörü yerine `match` kullanarak hatayı ele alıyoruz.
    let endpoint = match Endpoint::from_shared(whisper_url.clone()) {
        Ok(ep) => ep,
        Err(e) => {
            error!("Geçersiz uzman motor URL'si: {}. Hata: {}", whisper_url, e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"status": "unhealthy", "reason": "invalid_upstream_url"})));
        }
    };
    
    match endpoint.connect().await {
        Ok(_) => {
            info!("Sağlık durumu: Başarılı. Uzman motor erişilebilir.");
            (StatusCode::OK, Json(json!({"status": "healthy", "downstream_expert": "healthy"})))
        }
        Err(e) => {
            error!("Sağlık durumu: Başarısız. Uzman motora ({}) ulaşılamadı: {}", whisper_url, e);
            (StatusCode::SERVICE_UNAVAILABLE, Json(json!({"status": "unhealthy", "downstream_expert": "unreachable"})))
        }
    }
}