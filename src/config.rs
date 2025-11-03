use anyhow::{Context, Result};
use config::{Config, Environment};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    // Kendi dinleme adresleri için varsayılan değerler
    #[serde(default = "default_grpc_addr")]
    pub stt_gateway_service_grpc_listen_addr: SocketAddr,
    #[serde(default = "default_http_addr")]
    pub stt_gateway_service_http_listen_addr: SocketAddr,

    // Konuşacağı uzman motorun adresi (Bu zorunlu)
    pub stt_whisper_service_target_grpc_url: String,

    // Platform genelindeki standart değişkenler için varsayılanlar
    #[serde(default = "default_env")]
    pub env: String,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    
    // Build anında enjekte edilen meta veriler
    #[serde(default = "default_version")]
    pub service_version: String,
}

// Serde'nin `default` attribute'u için kullanılacak fonksiyonlar
fn default_grpc_addr() -> SocketAddr { "[::]:15021".parse().unwrap() }
fn default_http_addr() -> SocketAddr { "[::]:15020".parse().unwrap() }
fn default_env() -> String { "development".to_string() }
fn default_log_level() -> String { "debug".to_string() }
fn default_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let builder = Config::builder()
            // Önce `.env` dosyasını okumaya çalış (eğer varsa)
            .add_source(config::File::with_name(".env").required(false))
            // Sonra TÜM ortam değişkenlerini oku (öneksiz).
            // Bu, .env dosyasındaki değerleri ezer ve Docker için ana yöntemdir.
            .add_source(Environment::default().separator("_"));
        
        builder
            .build()?
            .try_deserialize()
            .context("Yapılandırma yüklenemedi. Gerekli ortam değişkenlerinin (örn: STT_WHISPER_SERVICE_TARGET_GRPC_URL) ayarlandığından emin olun.")
    }
}