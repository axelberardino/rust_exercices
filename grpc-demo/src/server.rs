use tonic::{transport::Server, Request, Response, Status};

use echo::echo_service_server::{EchoService, EchoServiceServer};
use echo::{SayRequest, SayResponse};

pub mod echo {
    tonic::include_proto!("echo");
}

#[derive(Debug, Default)]
pub struct MyEchoService {}

#[tonic::async_trait]
impl EchoService for MyEchoService {
    async fn say(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        println!("Got a request: {:?}", request);

        let resp = echo::SayResponse {
            message: format!("Hello {}!", request.into_inner().message).into(),
        };

        Ok(Response::new(resp))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let echo_service = MyEchoService::default();

    Server::builder()
        .add_service(EchoServiceServer::new(echo_service))
        .serve(addr)
        .await?;

    Ok(())
}
