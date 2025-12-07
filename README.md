# 👂 Sentiric STT Gateway Service

Platformun **Konuşma Tanıma (Speech-to-Text)** giriş noktasıdır. Canlı ses akışlarını (gRPC stream) alır ve `stt-whisper-service` (C++) gibi uzman motorlara iletir.

## 🚀 Özellikler
*   **Streaming Proxy:** İstemciden gelen ses paketlerini (chunk) anlık olarak motora iletir.
*   **Yük Dengeleme:** (Gelecek) Birden fazla Whisper worker'ı arasında yükü dağıtır.
*   **Protokol Soyutlama:** Arka planda farklı motorlar olsa bile dışarıya tek bir API sunar.

## 📦 Kurulum
```bash
make setup
make up
```

## 🔌 API
*   **gRPC (15011):** `sentiric.stt.v1.SttGatewayService`