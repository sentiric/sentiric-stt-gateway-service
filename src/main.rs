// src/main.rs - TAM VE SON HALİ

// Sentiric Contracts kütüphanesinden ihtiyacımız olan modülleri doğrudan `use` ile çağırıyoruz.
// Artık `mod grpc_generated;` gibi bir şeye gerek yok, çünkü kütüphane bunu kendisi yapıyor.
use sentiric_contracts::sentiric::stt::v1::{
    stt_gateway_service_server::{SttGatewayService, SttGatewayServiceServer},
    TranscribeRequest, TranscribeResponse, TranscribeStreamRequest, TranscribeStreamResponse,
};

use axum::{routing::get, Router};
use std::net::SocketAddr; // <-- BU IMPORT GEREKLİ
use tonic::transport::Server;
use tonic::{Request, Response, Status, Streaming};
use tokio_stream::Stream;

// Gateway servisimizin state'ini tutacak yapı.
#[derive(Debug, Default)]
pub struct MySttGateway {}

// gRPC servis kontratını (trait) implemente ediyoruz.
#[tonic::async_trait]
impl SttGatewayService for MySttGateway {
    async fn transcribe(
        &self,
        request: Request<TranscribeRequest>,
    ) -> Result<Response<TranscribeResponse>, Status> {
        println!("gRPC Transcribe isteği alındı: {:?}", request.get_ref());
        let reply = TranscribeResponse {
            transcription: "Merhaba, bu tekil bir testtir.".into(),
        };
        Ok(Response::new(reply))
    }

    type TranscribeStreamStream =
        std::pin::Pin<Box<dyn Stream<Item = Result<TranscribeStreamResponse, Status>> + Send>>;

    async fn transcribe_stream(
        &self,
        request: Request<Streaming<TranscribeStreamRequest>>,
    ) -> Result<Response<Self::TranscribeStreamStream>, Status> {
        println!("gRPC TranscribeStream isteği alındı.");
        let output = async_stream::try_stream! {
            // İleride buraya gerçek akış mantığı gelecek
            yield TranscribeStreamResponse {
                partial_transcription: "Merhaba, bu bir stream testidir.".into(),
                is_final: true,
            };
        };
        Ok(Response::new(Box::pin(output) as Self::TranscribeStreamStream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Adresleri tanımla
    let grpc_addr: SocketAddr = "[::]:15021".parse()?;
    let http_addr: SocketAddr = "[::]:15020".parse()?;

    // gRPC sunucusunu hazırla
    let stt_gateway = MySttGateway::default();
    let grpc_server = SttGatewayServiceServer::new(stt_gateway);
    println!("✅ gRPC sunucusu {} adresinde başlatılıyor...", grpc_addr);
    let grpc_task = tokio::spawn(Server::builder().add_service(grpc_server).serve(grpc_addr));

    // HTTP sunucusunu hazırla
    let http_app = Router::new().route("/health", get(health_check));
    println!("✅ HTTP sağlık kontrolü http://{} adresinde başlatılıyor...", http_addr);
    
    // === DÜZELTME BURADA ===
    // `axum::serve` zaten bir Future döndürdüğü için, onu doğrudan spawn etmiyoruz.
    // Listener'ı `serve`'e verip, sonucunu spawn ediyoruz.
    let listener = tokio::net::TcpListener::bind(http_addr).await?;
    let http_task = tokio::spawn(async move {
        axum::serve(listener, http_app).await
    });
    
    // Her iki sunucunun da çalışmasını bekle. Birisi çökerse, diğeri de durur.
    let (grpc_result, http_result) = tokio::join!(grpc_task, http_task);

    // Hata varsa propagate et
    grpc_result??;
    http_result??;

    Ok(())
}

async fn health_check() -> &'static str {
    "ok"
}
