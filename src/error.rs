// Dosya: src/error.rs
use thiserror::Error;

#[allow(dead_code)] // Henüz kullanılmayan hata türleri için uyarıyı bastırır
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    #[error("Unknown error")]
    Unknown,
}