# 👂 Sentiric STT Gateway Service

[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![Language](https://img.shields.io/badge/language-Python-blue.svg)]()
[![Framework](https://img.shields.io/badge/framework-FastAPI-blueviolet.svg)]()

**Sentiric STT Gateway Service**, platformun Konuşma Tanıma (Speech-to-Text - STT) yeteneklerini merkezileştirir. Ses dosyalarını veya canlı ses akışlarını alır ve en uygun STT motoruna (yüksek doğruluk, düşük gecikme, maliyet optimizasyonu kriterlerine göre) yönlendirir.

Bu servis, TTS Gateway'e benzer şekilde, Agent'ın birden fazla STT motoruyla uğraşmasını engeller.

## 🎯 Temel Sorumluluklar

*   **Dinamik Model Yönlendirme:** Gelen sesin formatına veya yapılandırmaya göre Whisper, Google Speech API gibi uzmanlara yönlendirir.
*   **Protokol Yönetimi:** HTTP/gRPC üzerinden dosya yüklemelerini ve WebSocket/gRPC Streaming üzerinden canlı akışları destekler.
*   **Maliyet ve Performans Optimizasyonu:** Yerel motorlar (Whisper) ile bulut API'leri arasında seçim yapmayı optimize eder.

## 🛠️ Teknoloji Yığını

*   **Dil:** Python 3.11
*   **Web Çerçevesi:** FastAPI / Uvicorn (Yüksek I/O için)
*   **Streaming:** WebSocket (veya gRPC Stream)
*   **Bağımlılıklar:** `sentiric-contracts` v1.9.0

## 🔌 API Etkileşimleri

*   **Gelen (Sunucu):**
    *   `sentiric-agent-service` (gRPC/HTTP POST/WebSocket): `Transcribe`, `TranscribeStream` RPC'leri.
*   **Giden (İstemci):**
    *   `sentiric-stt-whisper-service` (HTTP/gRPC)
    *   `sentiric-stt-streaming-service` (gRPC Stream)

---
## 🏛️ Anayasal Konum

Bu servis, [Sentiric Anayasası'nın](https://github.com/sentiric/sentiric-governance) **AI Gateway Layer**'ında yer alan merkezi bir bileşendir.