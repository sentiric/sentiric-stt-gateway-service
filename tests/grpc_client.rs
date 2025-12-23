// Dosya: tests/grpc_client.rs
use std::env;
use std::time::Duration;
use tokio_stream::StreamExt;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};
use tonic::metadata::MetadataValue; // <-- EKLENDİ
use sentiric_contracts::sentiric::stt::v1::{
    stt_gateway_service_client::SttGatewayServiceClient,
    TranscribeStreamRequest,
};

// Sertifika yollarını host makine yapısına göre ayarlıyoruz
const CA_PATH: &str = "../sentiric-certificates/certs/ca.crt";
// Test için Gateway Client sertifikalarını kullanıyoruz
const CERT_PATH: &str = "../sentiric-certificates/certs/stt-gateway-service.crt";
const KEY_PATH: &str = "../sentiric-certificates/certs/stt-gateway-service.key";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = args.last().expect("Lütfen bir WAV dosyası yolu verin");

    println!("🔒 Güvenlik Katmanı (mTLS) Hazırlanıyor...");
    
    // 1. Sertifikaları Yükle
    let ca_cert = tokio::fs::read(CA_PATH).await.expect("CA sertifikası bulunamadı");
    let client_cert = tokio::fs::read(CERT_PATH).await.expect("Client sertifikası bulunamadı");
    let client_key = tokio::fs::read(KEY_PATH).await.expect("Client key bulunamadı");

    let ca = Certificate::from_pem(ca_cert);
    let identity = Identity::from_pem(client_cert, client_key);

    // 2. TLS Konfigürasyonu
    let tls = ClientTlsConfig::new()
        .domain_name("sentiric.cloud") 
        .ca_certificate(ca)
        .identity(identity);

    println!("🔌 STT Gateway'e bağlanılıyor (Port 15021)...");
    
    // 3. Bağlantı Kanalı
    let channel = Channel::from_static("https://127.0.0.1:15021")
        .tls_config(tls)?
        .connect()
        .await?;

    let mut client = SttGatewayServiceClient::new(channel);

    println!("🎤 '{}' dosyası okunuyor...", file_path);
    let mut reader = hound::WavReader::open(file_path)?;
    let samples: Vec<i16> = reader.samples::<i16>().collect::<Result<_, _>>()?;

    // 4. Streaming Başlat
    // 16kHz, Mono, 16-bit varsayıyoruz. 
    // Her chunk yaklaşık 200ms ses taşısın (16000 * 0.2 = 3200 sample -> 6400 byte)
    let chunk_size = 3200; 
    let chunks: Vec<Vec<i16>> = samples.chunks(chunk_size).map(|s| s.to_vec()).collect();

    println!("🚀 Akış Başlatılıyor ({} Paket)...", chunks.len());

    let stream = async_stream::stream! {
        for chunk in chunks {
            let mut buffer = Vec::with_capacity(chunk.len() * 2);
            for &sample in &chunk {
                buffer.extend_from_slice(&sample.to_le_bytes());
            }
            yield TranscribeStreamRequest { audio_chunk: buffer };
            
            // Gerçek zamanlı akışı simüle etmek için hafif bekleme
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    };

    // --- TRACE ID EKLEME BÖLÜMÜ ---
    let mut request = tonic::Request::new(stream);
    // Rastgele veya sabit bir Trace ID ekliyoruz
    request.metadata_mut().insert("x-trace-id", MetadataValue::from_static("prod-test-session-001"));
    // ------------------------------

    let response = client.transcribe_stream(request).await?;
    let mut response_stream = response.into_inner();

    println!("📝 Yanıtlar Dinleniyor...\n------------------------------------------------");

    while let Some(res) = response_stream.next().await {
        match res {
            Ok(msg) => {
                // Sadece dolu yanıtları veya final yanıtı yazdır
                if !msg.partial_transcription.trim().is_empty() || msg.is_final {
                    if msg.is_final {
                        println!("✅ [FINAL]: {}", msg.partial_transcription);
                    } else {
                        // Satır içi güncelleme efekti
                        print!("\r⏳ [PARTIAL]: {:<50}", msg.partial_transcription);
                        use std::io::Write;
                        std::io::stdout().flush().unwrap();
                    }
                }
            }
            Err(e) => eprintln!("\n❌ HATA: {}", e),
        }
    }
    
    println!("\n------------------------------------------------");
    println!("🎉 Test Tamamlandı.");
    Ok(())
}