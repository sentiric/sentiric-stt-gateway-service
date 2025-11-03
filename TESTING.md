# 🧪 STT Gateway Servisi - Entegrasyon Testi Rehberi

Bu servis, `stt-whisper-service` gibi aşağı akış (downstream) uzman motorlarla entegrasyonunu test etmek için kendi izole Docker Compose ortamına sahiptir. Bu rehber, test ortamını nasıl kuracağınızı ve çalıştıracağınızı adım adım açıklar.

## 1. Ortamı Başlatma

Öncelikle, terminalde projenin kök dizinindeyken aşağıdaki komutu çalıştırarak test ortamını ayağa kaldırın:

```bash
docker compose -f docker-compose.dev.yml up --build -d
```
*Not: `stt-whisper-service`, ilk çalıştırmada model dosyalarını (`tiny` modeli) indirecektir. Bu işlem birkaç dakika sürebilir. `docker compose logs -f stt-whisper-service` komutuyla "Whisper model loaded successfully" mesajını görerek hazır olduğunu teyit edebilirsiniz.*

## 2. Test İstemcisini Çalıştırma

Test istemcisi, `Cargo`'nun test altyapısı kullanılarak çalıştırılır.

### Yöntem 1: Tek Komut ile Çalıştırma (Önerilen)

Aşağıdaki komut, `grpc_client` testini derler ve çalıştırır. `cargo test`'ten sonra gelen `--` ayıracının, argümanları doğrudan test programına iletmek için **zorunlu** olduğuna dikkat edin.

```bash
cargo test --test grpc_client -- -- /path/to/your/audio.wav
```

**Örnek:**```bash
cargo test --test grpc_client -- -- ../sentiric-assets/audio/tr/system/welcome_anonymous.wav
```

### Yöntem 2: Derle ve Çalıştır (Alternatif)

Eğer argümanlarla ilgili bir sorun yaşarsanız, testi iki adımda çalıştırabilirsiniz:

1.  **Testi Derle:**
    ```bash
    cargo test --test grpc_client --no-run
    ```

2.  **Derlenmiş Binary'yi Çalıştır:**
    ```bash
    # Aşağıdaki komut, en son derlenen test dosyasını bulup otomatik olarak çalıştırır.
    ./target/debug/deps/grpc_client-$(ls -t target/debug/deps | grep '^grpc_client-' | head -n 1 | cut -d- -f2- | cut -d. -f1) ../sentiric-assets/audio/tr/system/welcome_anonymous.wav
    ```

## 3. Logları İzleme (Hata Ayıklama için)

Test sırasında servislerin davranışını canlı olarak izlemek için yeni bir terminal açıp aşağıdaki komutları kullanabilirsiniz:

```bash
# Gateway logları
docker compose -f docker-compose.dev.yml logs -f stt-gateway-service

# Whisper motoru logları
docker compose -f docker-compose.dev.yml logs -f stt-whisper-service```

## 4. Ortamı Kapatma

Testleriniz bittiğinde, aşağıdaki komutla test ortamını ve ilgili tüm kaynakları temiz bir şekilde kapatın:

```bash
docker compose -f docker-compose.dev.yml down --volumes
```
*`--volumes` bayrağı, `stt_whisper_cache` gibi Docker volume'lerini de siler. Modeli tekrar indirmek istemiyorsanız bu bayrağı kaldırabilirsiniz.*

---

## ⚡ Performans Analizi: CPU vs. GPU

Bu test ortamı, `stt-whisper-service`'in farklı donanımlar üzerindeki performansını karşılaştırmak için de kullanılabilir. Aşağıdaki sonuçlar, `welcome_anonymous.wav` (~10 saniyelik) ses dosyası ve `tiny` Whisper modeli kullanılarak yapılan bir testten alınmıştır.

### Test Sonuçları

| Metrik | CPU (`int8`) | GPU (`float16`) | Performans Artışı |
| :--- | :--- | :--- | :--- |
| **Whisper Net İşlem Süresi** | ~650 ms | **~212 ms** | **~3.1x Hızlanma** |
| **Gateway'in Eklediği Gecikme** | ~14 ms | ~5 ms | - |
| **Toplam Uçtan Uca Gecikme** | ~664 ms | **~217 ms** | **~3.1x Hızlanma** |

```mermaid
barChart
    title: STT İşlem Süresi Karşılaştırması (tiny model)
    "CPU (int8)": 650
    "GPU (float16)": 212```

### Analiz ve Çıkarımlar

1.  **Donanım Etkisi:** `stt-whisper-service`'i bir GPU üzerinde çalıştırmak, aynı iş yükü için transkripsiyon süresini **3 kattan fazla** iyileştirmiştir. Bu, düşük gecikmenin kritik olduğu senaryolar için GPU kullanımının önemini göstermektedir.

2.  **Gateway Verimliliği:** Her iki senaryoda da `stt-gateway-service`'in eklediği gecikme (overhead) **~15 milisaniyenin altındadır**. Bu, Rust ile geliştirilen gateway'in son derece hafif ve verimli olduğunu, toplam işlem süresine ihmal edilebilir bir etki yaptığını kanıtlamaktadır.

3.  **Darboğaz:** Performans darboğazı, beklendiği gibi, yapay zeka modelinin çalıştığı `stt-whisper-service`'dir. Performans optimizasyonları bu uzman motor üzerinde yoğunlaşmalıdır.

Bu sonuçlar, `stt-gateway-service`'in mimari hedeflerini başarıyla karşıladığını ve platformun hibrit donanım yeteneklerini tam olarak desteklediğini doğrulamaktadır.