# 📋 Teknik Şartname

## 1. Servis Kimliği
*   **Adı:** `sentiric-stt-gateway-service`
*   **Dil:** Rust
*   **Port:** 15011 (gRPC)

## 2. Performans
*   **Latency:** Proxy işlemi < 1ms ek gecikme yaratır.
*   **Memory:** Yük altında bile sabit bellek kullanımı (Streaming sayesinde).

## 3. Kontrat
*   `sentiric.stt.v1.SttGatewayService`
*   Method: `TranscribeStream`