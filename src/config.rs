use config::{Config, File, Environment};
use serde::Deserialize;
use anyhow::Result;
use std::env;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub env: String,
    pub rust_log: String,
    pub service_version: String,
    
    // Ağ Ayarları (Standardize)
    pub host: String,      // STT_GATEWAY_SERVICE_LISTEN_ADDRESS
    pub http_port: u16,    // STT_GATEWAY_SERVICE_HTTP_PORT
    pub grpc_port: u16,    // STT_GATEWAY_SERVICE_GRPC_PORT

    // Upstream Services
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
            
            // 1. MANUAL OVERRIDES (Standart Değişken Eşleştirme)
            .set_override_option("host", env::var("STT_GATEWAY_SERVICE_LISTEN_ADDRESS").ok())?
            .set_override_option("http_port", env::var("STT_GATEWAY_SERVICE_HTTP_PORT").ok())?
            .set_override_option("grpc_port", env::var("STT_GATEWAY_SERVICE_GRPC_PORT").ok())?
            .set_override_option("stt_whisper_service_grpc_url", env::var("STT_WHISPER_SERVICE_GRPC_URL").ok())?
            
            // 2. SMART DEFAULTS (Zero-Config İçin)
            .set_default("env", "production")?
            .set_default("rust_log", "info,sentiric_stt_gateway=debug")?
            .set_default("service_version", "1.1.0")?
            
            .set_default("host", "0.0.0.0")?
            .set_default("http_port", 15020)?
            .set_default("grpc_port", 15021)?
            
            // Varsayılan Hedef (mTLS için https şart)
            .set_default("stt_whisper_service_grpc_url", "https://stt-whisper-service:15031")?

            // Varsayılan Sertifika Yolları (Container Standardı)
            .set_default("grpc_tls_ca_path", "/sentiric-certificates/certs/ca.crt")?
            .set_default("stt_gateway_service_cert_path", "/sentiric-certificates/certs/stt-gateway-service.crt")?
            .set_default("stt_gateway_service_key_path", "/sentiric-certificates/certs/stt-gateway-service.key")?;

        builder.build()?.try_deserialize().map_err(|e| e.into())
    }
}