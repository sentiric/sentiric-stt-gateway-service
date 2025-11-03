use sentiric_contracts::sentiric::stt::v1::{
    stt_gateway_service_server::SttGatewayService,
    TranscribeRequest, TranscribeResponse, TranscribeStreamRequest, TranscribeStreamResponse,
};
use tokio_stream::Stream;
use tonic::{Request, Response, Status, Streaming};

#[derive(Debug, Default)]
pub struct MySttGateway {}

#[tonic::async_trait]
impl SttGatewayService for MySttGateway {
    async fn transcribe(
        &self,
        _request: Request<TranscribeRequest>,
    ) -> Result<Response<TranscribeResponse>, Status> {
        Err(Status::unimplemented("Tekil transcribe henüz implemente edilmedi."))
    }

    type TranscribeStreamStream =
        std::pin::Pin<Box<dyn Stream<Item = Result<TranscribeStreamResponse, Status>> + Send>>;

    async fn transcribe_stream(
        &self,
        _request: Request<Streaming<TranscribeStreamRequest>>,
    ) -> Result<Response<Self::TranscribeStreamStream>, Status> {
        Err(Status::unimplemented("Stream transcribe henüz implemente edilmedi."))
    }
}