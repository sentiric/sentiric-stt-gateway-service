# 🧠 Akış Mantığı (Streaming Logic)

Bu servis, Rust'ın `Tokio` asenkron çalışma zamanı ve `Tonic` gRPC kütüphanesini kullanarak yüksek performanslı bir "Streaming Proxy" görevi görür.

## Veri Akış Diyagramı

```mermaid
sequenceDiagram
    participant Client
    participant Gateway
    participant Whisper

    Client->>Gateway: gRPC Stream Start (mTLS)
    Gateway->>Whisper: gRPC Stream Start (mTLS)

    par Audio Flow
        loop Every 20ms
            Client->>Gateway: Audio Chunk
            Gateway->>Whisper: Audio Chunk (Forwarded)
        end
    and Text Flow
        loop Asynchronous
            Whisper-->>Gateway: Partial Transcript
            Gateway-->>Client: Partial Transcript
        end
    end
```

## Stream Dönüşümü (Mapping)

Gateway, iki farklı proto mesajı arasında çeviri yapar:
*   **Girdi:** `TranscribeStreamRequest` -> `WhisperTranscribeStreamRequest`
*   **Çıktı:** `WhisperTranscribeStreamResponse` -> `TranscribeStreamResponse`

Bu işlem `src/grpc/server.rs` dosyasında `filter_map` ve `map` fonksiyonları ile reaktif (reactive) olarak yapılır.
