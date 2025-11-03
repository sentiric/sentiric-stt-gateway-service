use anyhow::{Context, Result};
use config::{Config, Environment};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_grpc_addr")]
    pub grpc_listen_addr: SocketAddr,
    #[serde(default = "default_http_addr")]
    pub http_listen_addr: SocketAddr,
    pub stt_whisper_service_url: String, // Örn: "http://localhost:15031"
    #[serde(default = "default_env")]
    pub env: String,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_version")]
    pub service_version: String,
}

fn default_grpc_addr() -> SocketAddr { "[::]:15021".parse().unwrap() }
fn default_http_addr() -> SocketAddr { "[::]:15020".parse().unwrap() }
fn default_env() -> String { "production".to_string() }
fn default_log_level() -> String { "info".to_string() }
fn default_version() -> String { "0.1.0".to_string() }

impl AppConfig {
    pub fn load() -> Result<Self> {
        let builder = Config::builder()
            .add_source(Environment::with_prefix("SENTIRIC").separator("__").try_parsing(true))
            .set_default("service_version", env!("CARGO_PKG_VERSION"))?;
        
        builder.build()?.try_deserialize().context("Yapılandırma okunamadı")
    }
}