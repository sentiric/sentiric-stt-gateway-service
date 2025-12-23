# 👂 Sentiric STT Gateway Service

[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![Security](https://img.shields.io/badge/security-mTLS-green.svg)]()
[![Protocol](https://img.shields.io/badge/protocol-BiDirectional_Stream-orange.svg)]()

**Sentiric İletişim İşletim Sistemi**'nin "İşitme Merkezi"dir. Platforma giren tüm canlı ses akışlarını (Audio Streams) karşılar, mTLS tüneli üzerinden güvenli bir şekilde Whisper Motoruna iletir ve anlık transkripsiyonları geri döndürür.

## 🎯 Temel Yetenekler

1.  **Çift Yönlü Akış (Bi-Directional Streaming):** İstemci ses gönderirken aynı anda sunucu metin gönderebilir. Tam full-duplex iletişim.
2.  **Sıfır Kopya (Zero-Copy Proxy):** Gelen ses paketlerini bellekte biriktirmeden veya işlemeden doğrudan motora aktarır. Minimal gecikme.
3.  **Güvenlik:** Tüm iletişim mTLS ile şifrelidir.

## 🏗️ Mimari Konum

*   **Üst Akış (Caller):** `telephony-action-service`
*   **Alt Akış (Upstream):** `stt-whisper-service` (C++ / GPU)

## 📦 Kurulum ve Ortam Değişkenleri

```bash
# .env Örneği
HOST=0.0.0.0
GRPC_PORT=15011

# Hedef Motor
STT_WHISPER_URL=http://stt-whisper-service:15031

# Güvenlik
GRPC_TLS_CA_PATH=../sentiric-certificates/certs/ca.crt
STT_GATEWAY_SERVICE_CERT_PATH=../sentiric-certificates/certs/stt-gateway-service.crt
STT_GATEWAY_SERVICE_KEY_PATH=../sentiric-certificates/certs/stt-gateway-service.key
```