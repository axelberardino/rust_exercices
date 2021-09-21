use echo::echo_service_client::EchoServiceClient;
use echo::SayRequest;

pub mod echo {
    tonic::include_proto!("echo");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EchoServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(SayRequest {
        message: "Tonic".into(),
    });

    let response = client.say(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
