# 🧠 Akış Mantığı

1.  **TranscribeStream (gRPC):** İstemci (Telephony Action Service) bir stream başlatır.
2.  **Upstream Bağlantısı:** Gateway, `STT_WHISPER_SERVICE_GRPC_URL` adresine bir stream açar.
3.  **Pipe:** Gelen her `audio_chunk` paketini değişikliğe uğratmadan upstream'e yazar (Zero-copy hedeflenir).
4.  **Response:** Upstream'den gelen `Transcript` olaylarını istemciye geri döner.