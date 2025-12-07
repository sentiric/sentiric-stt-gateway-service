# 👂 STT Gateway Service - Görev Listesi

Bu liste, bu repoyu devralacak geliştirici için öncelikli işleri sıralar.

## 🔴 Faz 1: İskelet ve Contract Entegrasyonu
- [ ] **Protobuf Entegrasyonu:** `sentiric-contracts` reposunu ekle ve `build.rs` ile derle.
- [ ] **gRPC Server:** `tonic` ile `RecognizeStream` metodunun iskeletini oluştur.

## 🟡 Faz 2: Whisper Entegrasyonu
- [ ] **Whisper Client:** `stt-whisper-service`'e gRPC stream açan bir `WhisperClient` struct'ı yaz.
- [ ] **Stream Forwarding:** İstemciden gelen `audio_chunk`ları, üzerinde işlem yapmadan (zero-copy) Whisper Client'ın stream'ine aktar.
- [ ] **Response Handling:** Whisper'dan gelen yanıtları `RecognizeStreamResponse` formatına çevirip istemciye dön.

## 🟢 Faz 3: Routing ve Config
- [ ] **Config Handling:** İlk mesajın `StreamingConfig` olup olmadığını kontrol et. Değilse hata dön.
- [ ] **Router:** `config.model_preference` alanına göre doğru client'ı (şimdilik sadece Whisper) seçen bir mantık ekle.

## 🔵 Faz 4: Performans ve Güvenlik
- [ ] **Concurrency:** `tokio::select!` veya `stream types` kullanarak bi-directional akışı kilitlemeden (non-blocking) yönet.
- [ ] **mTLS:** Güvenli bağlantıyı aktif et.