# 📋 Teknik Şartname (Specification)

## 1. Servis Kimliği
*   **Adı:** `sentiric-stt-gateway-service`
*   **Dil:** Rust (Tokio / Tonic)
*   **Port Bloğu:** 1502X (Harmonik Mimari)

## 2. API Kontratı (gRPC)

Servis, `sentiric-contracts` reposundaki `sentiric.stt.v1` paketini implemente eder.

### Proto Tanımı (`stt/v1/gateway.proto`)

```protobuf
service SttGatewayService {
  // Bi-directional streaming RPC
  rpc RecognizeStream(stream RecognizeStreamRequest) returns (stream RecognizeStreamResponse);
}

message RecognizeStreamRequest {
  oneof payload {
    StreamingConfig config = 1; // İlk mesajda zorunlu
    bytes audio_chunk = 2;      // Sonraki mesajlarda ses verisi
  }
}

message StreamingConfig {
  string language_code = 1;
  string model_preference = 2; // "whisper", "google"
  int32 sample_rate = 3;       // Genelde 8000 veya 16000
}

message RecognizeStreamResponse {
  string transcript_chunk = 1;
  bool is_final = 2;           // Cümle sonu mu?
  string engine_used = 3;
}
```

## 3. Ortam Değişkenleri

| Değişken | Zorunlu | Açıklama |
| :--- | :--- | :--- |
| `STT_GATEWAY_SERVICE_GRPC_PORT` | Evet | 15021 |
| `STT_WHISPER_SERVICE_GRPC_URL` | Evet | http://stt-whisper-service:15031 |
| `RUST_LOG` | Evet | Log seviyesi (info/debug) |

## 4. Performans Hedefleri

*   **Latency:** Gateway'in eklediği gecikme (overhead) < 5ms olmalıdır.
*   **Concurrency:** Tek bir pod, 100+ eş zamanlı ses akışını (stream) bellek şişmesi yaşamadan yönetebilmelidir.