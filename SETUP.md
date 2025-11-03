# 🚀 STT Gateway Service - Yerel Geliştirme Ortamı Kurulumu

## 1. Önkoşullar
- Rust (rustup ile)
- `protobuf-compiler`
    ```bash
    # Debian/Ubuntu için
    sudo apt update && sudo apt install -y protobuf-compiler
    ```

## 2. Projeyi Derleme ve Çalıştırma
1. Repoyu klonlayın.
2. `cargo build` komutu ile bağımlılıkları indirin ve projeyi derleyin.
3. `cargo run` komutu ile servisi başlatın.

## 3. Test Etme
- **Health Check:** `curl http://localhost:15020/health`
- **gRPC:** `grpcurl -plaintext localhost:15021 list`

---
