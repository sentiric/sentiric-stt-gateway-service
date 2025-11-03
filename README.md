# 👂 Sentiric STT Gateway Service

[![CI Status](https://github.com/sentiric/sentiric-stt-gateway-service/actions/workflows/docker-publish.yml/badge.svg)](https://github.com/sentiric/sentiric-stt-gateway-service/actions/workflows/docker-publish.yml)
[![Language](https://img.shields.io/badge/language-Rust-orange.svg)]()

**Sentiric STT Gateway Service**, platformun Konuşma Tanıma (Speech-to-Text - STT) yeteneklerini merkezileştiren, yüksek performanslı ve dayanıklı bir yönlendiricidir. `agent-service` gibi iç servislerden gelen ses transkripsiyon isteklerini alır ve bu istekleri en uygun "uzman" STT motoruna (`stt-whisper-service`, gelecekte `stt-google-service` vb.) akıllıca yönlendirir.

Bu servis, `tts-gateway-service` gibi, orkestrasyon katmanının birden fazla STT motorunun karmaşıklığıyla uğraşmasını engeller ve tek, tutarlı bir arayüz sunar.

## 🎯 Temel Sorumluluklar

*   **Protokol Soyutlama:** Gelen tüm istekleri standart bir gRPC arayüzü üzerinden kabul eder.
*   **Akıllı Yönlendirme (Routing):** Gelecekte, gelen isteğin `tenant_id`'sine, `model_selector`'a veya yapılandırmaya göre en uygun uzman STT motorunu (örn: maliyet için Whisper, en yüksek doğruluk için Google) seçecektir.
*   **Yük Dengeleme (Load Balancing):** Gelecekte, aynı türden birden fazla uzman motor arasında yükü dağıtabilecektir.
*   **Dayanıklılık (Resilience):** Bir uzman motor çöktüğünde, isteği otomatik olarak çalışan bir yedeğe yönlendirme (fallback) yeteneğine sahip olacaktır.

## 🛠️ Teknoloji Yığını

*   **Dil:** Rust
*   **Asenkron Runtime:** Tokio
*   **Servisler Arası İletişim:** gRPC (Tonic ile)
*   **Web Sunucusu (Health Check için):** Axum

## 🔌 API Etkileşimleri

*   **Gelen (Sunucu):**
    *   `sentiric-agent-service` veya `sentiric-telephony-action-service` (gRPC): `TranscribeStream` RPC'sini çağırır.
*   **Giden (İstemci):**
    *   `sentiric-stt-whisper-service` (gRPC)
    *   (Gelecekte) Diğer `stt-*` uzman motorları.

---
## 🏛️ Anayasal Konum

Bu servis, [Sentiric Anayasası'nın](https://github.com/sentiric/sentiric-governance/blob/main/docs/blueprint/Architecture-Overview.md) **AI Gateway Layer**'ında yer alan merkezi bir bileşendir.