# 🧪 STT Gateway Servisi - Entegrasyon Testi Rehberi

Bu servis, `stt-whisper-service` gibi aşağı akış (downstream) uzman motorlarla entegrasyonunu test etmek için kendi izole Docker Compose ortamına sahiptir.

## Hızlı Başlangıç

1.  **Ortamı Başlat:**
    ```bash
    docker compose -f docker-compose.dev.yml up --build -d
    ```
    *Not: `stt-whisper-service` ilk çalıştırmada model dosyalarını indirecektir, bu işlem birkaç dakika sürebilir.*

2.  **Test İstemcisini Çalıştır:**
    Projenin kök dizinindeyken, test etmek istediğiniz bir `.wav` dosyasının yolunu belirterek aşağıdaki komutu çalıştırın:
    ```bash
    cargo run --test grpc_client -- /path/to/your/audio.wav
    ```
    Örnek:
    ```bash
    cargo run --test grpc_client -- ../sentiric-assets/audio/tr/system/welcome_anonymous.wav
    ```

3.  **Logları İzle (İsteğe Bağlı):**
    ```bash
    # Gateway logları
    docker compose -f docker-compose.dev.yml logs -f stt-gateway-service

    # Whisper motoru logları
    docker compose -f docker-compose.dev.yml logs -f stt-whisper-service
    ```

4.  **Ortamı Kapat:**
    ```bash
    docker compose -f docker-compose.dev.yml down
    ```