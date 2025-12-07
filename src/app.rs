use crate::config::AppConfig;
use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub struct App {
    config: AppConfig,
}

impl App {
    pub async fn bootstrap() -> Result<Self> {
        dotenvy::dotenv().ok();
        let config = AppConfig::load()?;

        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(&config.rust_log))
            .with(fmt::layer())
            .init();

        info!("🚀 STT Gateway Service v{} başlatılıyor...", config.service_version);
        Ok(Self { config })
    }

    pub async fn run(self) -> Result<()> {
        info!("Servisler ayağa kaldırılıyor (HTTP: {}, gRPC: {})...", 
              self.config.http_port, self.config.grpc_port);
        
        info!("Upstream: Whisper Engine @ {}", self.config.stt_whisper_url);

        tokio::signal::ctrl_c().await?;
        info!("🛑 Kapatılıyor...");
        Ok(())
    }
}