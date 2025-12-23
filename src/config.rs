// Dosya: src/config.rs
use config::{Config, File, Environment};
use serde::Deserialize;
use anyhow::Result;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub env: String,
    pub rust_log: String,
    pub service_version: String, // <-- EKLENEN EKSİK ALAN
    
    pub host: String,
    pub grpc_port: u16,

    // Upstream Services
    // STT Whisper Service C++ Engine (gRPC)
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
            
            // Manuel Override
            .set_override_option("stt_whisper_service_grpc_url", std::env::var("STT_WHISPER_SERVICE_GRPC_URL").ok())?
            
            // Varsayılan Değerler
            .set_default("env", "development")?
            .set_default("rust_log", "info")?
            .set_default("service_version", "1.1.0")? // Burası vardı ama struct'ta yoktu
            .set_default("host", "0.0.0.0")?
            .set_default("grpc_port", 15021)?
            .set_default("stt_whisper_service_grpc_url", "https://stt-whisper-service:15031")?;

        builder.build()?.try_deserialize().map_err(|e| e.into())
    }
}