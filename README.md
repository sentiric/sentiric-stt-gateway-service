# 👂 Sentiric STT Gateway Service

[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![Architecture](https://img.shields.io/badge/architecture-layer_3_gateway-blue.svg)]()
[![Language](https://img.shields.io/badge/language-Rust-orange.svg)]()

**Sentiric İletişim İşletim Sistemi**'nin "İşitme Merkezi"dir. Platforma giren tüm canlı ses akışlarını (Audio Streams) karşılar ve bunları analiz edilmesi için uygun "Uzman Motorlara" (Whisper, Google STT vb.) yönlendirir.

## 🎯 Temel Sorumluluklar

1.  **Akış Yönetimi (Bi-Directional Streaming):** İstemciden gelen ses parçalarını (chunks) alıp motora iletirken, motordan gelen metin parçalarını (transcripts) anlık olarak istemciye iletir.
2.  **Akıllı Yönlendirme:** İsteğin `language_code` veya `model_preference` parametrelerine göre trafiği `stt-whisper-service` (Yerel) veya bulut sağlayıcılara yönlendirir.
3.  **Protokol Dönüşümü:** İç gRPC formatını, hedef motorun beklediği formata (gRPC veya WebSocket) dönüştürür.
4.  **Yük Dengeleme (Load Balancing):** Birden fazla Whisper işçisi (worker) varsa, yükü aralarında dağıtır (Gelecek özellik).

## 🏗️ Mimari Konum

Bu servis **Katman 3 (Ağ Geçitleri)** seviyesinde yer alır.

*   **Üst Akış (Callers):** `telephony-action-service`.
*   **Alt Akış (Downstreams):**
    *   `stt-whisper-service` (C++ / GPU / Yerel / gRPC)
    *   *(Opsiyonel)* Google Speech-to-Text (Bulut / REST)

## 📦 Kurulum ve Çalıştırma

### Gereksinimler
*   Rust (1.75+)
*   Protobuf Compiler (`protoc`)

### Komutlar
```bash
# Ortamı hazırla
make setup

# Servisi başlat
make up

# Logları izle
make logs
```

## 🔌 API ve Portlar

*   **gRPC (15011):** `sentiric.stt.v1.SttGatewayService`
*   **HTTP (15010):** `/health`, `/metrics`