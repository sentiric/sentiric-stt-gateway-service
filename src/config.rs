use anyhow::{Context, Result};
use config::{Config, Environment};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    // Kendi dinleme adresleri
    #[serde(rename = "stt_gateway_service_grpc_listen_addr")]
    pub grpc_listen_addr: SocketAddr,
    #[serde(rename = "stt_gateway_service_http_listen_addr")]
    pub http_listen_addr: SocketAddr,

    // Konuşacağı uzman motorun adresi
    #[serde(rename = "stt_whisper_service_target_grpc_url")]
    pub stt_whisper_service_target_grpc_url: String,

    // Platform genelindeki standart değişkenler
    pub env: String,
    pub log_level: String,
    
    // Build anında enjekte edilen meta veriler
    #[serde(default = "default_version")]
    pub service_version: String,
}

fn default_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let builder = Config::builder()
            // Tüm ortam değişkenlerini, belirli bir önek olmadan okur.
            .add_source(Environment::default().separator("__"));
        
        builder
            .build()?
            .try_deserialize()
            .context("Ortam değişkenlerinden yapılandırma okunamadı")
    }
}