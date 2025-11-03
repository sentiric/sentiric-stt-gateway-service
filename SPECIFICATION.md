# Sentiric STT Gateway Servisi - Teknik Şartname

**Belge Versiyonu:** 1.0
**Durum:** Geliştirme Aşamasında

## 1. Genel Bakış
Bu servis, platformun Konuşma Tanıma (STT) yetenekleri için merkezi bir giriş noktasıdır. `agent-service` gibi iç servislerden gelen gRPC isteklerini alır ve bunları `stt-whisper-service` gibi uygun uzman AI motorlarına yönlendirir.

## 2. Temel Yetenekler
- **Protokol Soyutlama:** Tüm STT işlemlerini tek bir standart gRPC arayüzü (`SttGatewayService`) arkasında birleştirir.
- **Akıllı Yönlendirme (Gelecek):** İstek içeriğine göre en uygun uzman motoru (Whisper, Google vb.) seçer.
- **Gerçek Zamanlı Akış:** Canlı ses akışlarını, uzman motorlara çift yönlü olarak iletir.

## 3. API Arayüzü
- **gRPC:** `sentiric.stt.v1.SttGatewayService` kontratını implemente eder.
- **HTTP:** Sadece `/health` endpoint'ini sunar.