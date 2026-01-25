use config::{Config, File, Environment};
use serde::Deserialize;
use anyhow::Result;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    #[allow(dead_code)]
    pub env: String,
    pub rust_log: String,
    pub service_version: String,
    
    // Ağ Ayarları
    pub host: String,
    pub grpc_port: u16,
    pub http_port: u16, // EKLENDİ

    // Upstream
    pub stt_whisper_service_grpc_url: String, 

    // Security
    pub grpc_tls_ca_path: String,
    pub stt_gateway_service_cert_path: String,
    pub stt_gateway_service_key_path: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let builder = Config::builder()
            .add_source(File::with_name(".env").required(false))
            .add_source(Environment::default().separator("__"))
            
            // MANUAL OVERRIDES
            .set_override_option("host", env::var("STT_GATEWAY_SERVICE_LISTEN_ADDRESS").ok())?
            .set_override_option("grpc_port", env::var("STT_GATEWAY_SERVICE_GRPC_PORT").ok())?
            .set_override_option("http_port", env::var("STT_GATEWAY_SERVICE_HTTP_PORT").ok())? // EKLENDİ
            .set_override_option("stt_whisper_service_grpc_url", env::var("STT_WHISPER_SERVICE_GRPC_URL").ok())?
            
            // DEFAULTS
            .set_default("env", "production")?
            .set_default("rust_log", "info,sentiric_stt_gateway=debug")?
            .set_default("service_version", "1.1.2")?
            
            .set_default("host", "0.0.0.0")?
            .set_default("grpc_port", 15021)?
            .set_default("http_port", 15020)? // EKLENDİ
            
            .set_default("stt_whisper_service_grpc_url", "https://stt-whisper-service:15031")?

            .set_default("grpc_tls_ca_path", "/sentiric-certificates/certs/ca.crt")?
            .set_default("stt_gateway_service_cert_path", "/sentiric-certificates/certs/stt-gateway-service.crt")?
            .set_default("stt_gateway_service_key_path", "/sentiric-certificates/certs/stt-gateway-service.key")?;

        builder.build()?.try_deserialize().map_err(|e| e.into())
    }
}