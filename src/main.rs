use sentiric_stt_gateway_service::{
    config::AppConfig,
    grpc::service::MySttGateway,
};
use anyhow::Result;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use sentiric_contracts::sentiric::stt::v1::stt_gateway_service_server::SttGatewayServiceServer;
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use tonic::transport::Server;
use tracing::{error, info, instrument};
use tracing_subscriber::{fmt, EnvFilter, Registry};
use tracing_subscriber::prelude::*;

#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    // Ortam değişkenlerinden yapılandırmayı yükle
    let config = Arc::new(AppConfig::load()?);

    // Ortama duyarlı loglama altyapısını kur
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

    // gRPC ve HTTP sunucularını ayrı görevlerde başlat
    let grpc_task = tokio::spawn(run_grpc_server(config.clone()));
    let http_task = tokio::spawn(run_http_server(config.clone()));

    // Herhangi bir sunucu hata ile çökerse, tüm programı sonlandır
    tokio::try_join!(grpc_task, http_task)??;
    
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
    match tonic::transport::Endpoint::from_shared(whisper_url.clone())?.connect().await {
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