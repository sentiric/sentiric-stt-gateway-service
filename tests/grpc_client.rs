use std::env;
use tokio_stream::StreamExt;
use sentiric_contracts::sentiric::stt::v1::{
    stt_gateway_service_client::SttGatewayServiceClient,
    TranscribeStreamRequest,
};

// Bu test, bir WAV dosyasını okuyup gateway'e stream eder ve sonuçları yazdırır.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Kullanım: cargo run --test grpc_client -- <wav_dosyasi_yolu>");
        // DÜZELTME: Hatalı kullanımda başarılı (0) yerine hata kodu (1) ile çık.
        std::process::exit(1);
    }
    let file_path = &args[1];

    println!("🔌 STT Gateway'e bağlanılıyor: http://127.0.0.1:15021");
    let mut client = SttGatewayServiceClient::connect("http://127.0.0.1:15021").await?;

    println!("🎤 '{}' dosyası okunuyor ve stream ediliyor...", file_path);

    let mut reader = hound::WavReader::open(file_path)?;
    // DÜZELTME: Kullanılmayan değişkeni `_spec` olarak işaretle.
    let _spec = reader.spec();
    
    let samples: Vec<i16> = reader.samples::<i16>().collect::<Result<_, _>>()?;

    // Ses dosyasını 8000 byte'lık (1 saniyelik 8kHz/16bit) parçalara ayır
    let chunk_size = 8000; 

    // DÜZELTME: Referans yerine verinin kopyasını taşı.
    // Her bir chunk'ı kendi `Vec<i16>`'ine dönüştürüyoruz.
    let chunks: Vec<Vec<i16>> = samples.chunks(chunk_size / 2).map(|s| s.to_vec()).collect();

    let stream = tokio_stream::iter(chunks.into_iter().map(|chunk| {
        // Her bir chunk'ı byte vektörüne dönüştür.
        let mut buffer = Vec::with_capacity(chunk.len() * 2);
        for &sample in &chunk {
            buffer.extend_from_slice(&sample.to_le_bytes());
        }
        TranscribeStreamRequest { audio_chunk: buffer }
    }));

    println!("🎧 Sunucudan transkripsiyon bekleniyor...");
    let mut response_stream = client.transcribe_stream(stream).await?.into_inner();

    let mut final_transcript = Vec::new();

    while let Some(res) = response_stream.next().await {
        match res {
            Ok(response) => {
                let text = response.partial_transcription.trim();
                if !text.is_empty() {
                    println!("   ↳ [Segment]: {}", text);
                    final_transcript.push(text.to_string());
                }
            }
            Err(e) => eprintln!("❌ Stream hatası: {}", e),
        }
    }
    
    println!("\n✅ Stream tamamlandı.");
    println!("====================");
    println!("Final Transkript: {}", final_transcript.join(" "));
    println!("====================");

    Ok(())
}