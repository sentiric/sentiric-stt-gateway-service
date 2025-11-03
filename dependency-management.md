# 🤫 Sentiric STT Gateway Service - Bağımlılık Yönetimi

Bu doküman, bu servisin kullandığı temel kütüphaneleri ve alınan mimari kararları özetler.

## 1. Temel Teknoloji Yığını
- **Dil:** Rust
- **Asenkron Runtime:** Tokio
- **API Framework'leri:**
    - gRPC: `tonic`
    - HTTP: `axum`

## 2. Mimari Karar: Neden Rust?
Bu servis, `tts-gateway-service`'te olduğu gibi, yüksek eşzamanlılık, güvenlik ve düşük kaynak tüketimi hedefleriyle Rust ile yazılmıştır. Detaylar için `governance` reposundaki `ADR-008`'e bakınız.

## 3. Kontrat Yönetimi
Servis, `sentiric-contracts` reposunu `Cargo.toml` üzerinden bir `git` bağımlılığı olarak kullanır. Bu, tüm API tanımlarının merkezi ve sürüm kontrollü kalmasını sağlar. Projenin kendisi `.proto` dosyalarını derlemez; bu işi `sentiric-contracts` kütüphanesi kendi `build.rs` script'i ile yapar.