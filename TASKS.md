# 🎯 STT Gateway Service - Görev Listesi

## ✅ Tamamlanan Görevler (v0.2.0)

- [x] **[ALTYAPI]** Servisin `sentiric-infrastructure`'a tam entegrasyonu.
- [x] **[YAPILANDIRMA]** `sentiric-config` üzerinden standartlaştırılmış merkezi yapılandırmanın okunması.
- [x] **[ÇEKİRDEK]** Gelen gRPC stream'ini `stt-whisper-service`'e şeffaf proxy olarak yönlendirme mantığının tamamlanması.
- [x] **[OPERASYON]** `ENV` değişkenine duyarlı, yapılandırılmış loglama (JSON/Metin) altyapısının kurulması.
- [x] **[OPERASYON]** Uzman motorun (downstream) gRPC bağlantısını kontrol eden derinlemesine `/health` endpoint'inin implemente edilmesi.
- [x] **[OPERASYON]** `Dockerfile` ve CI/CD pipeline'ının üretime hazır hale getirilmesi.
- [x] **[DOKÜMANTASYON]** README, LOGIC, SPECIFICATION ve SETUP belgelerinin oluşturulması.

## 🗺️ Gelecek Yol Haritası

- [ ] **[DAYANIKLILIK]** `ADR-007` ile uyumlu olarak Devre Kesici (Circuit Breaker) deseni implementasyonu.
- [ ] **[ZEKA]** İstek içeriğine göre farklı uzman motorlara (Google, Azure vb.) yönlendirme yapacak akıllı yönlendirici mantığı.
- [ ] **[PERFORMANS]** Birden fazla uzman motor örneği arasında yük dengeleme (Load Balancing) yeteneği.
- [ ] **[GÜVENLİK]** Tüm gRPC istemci ve sunucu bağlantılarında mTLS'in zorunlu hale getirilmesi.