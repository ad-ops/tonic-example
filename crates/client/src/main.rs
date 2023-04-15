mod client;

use crate::client::greeter::greeter_client::GreeterClient;
use crate::client::greeter::HelloRequest;
use color_eyre::eyre::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_level(true)
        .with_file(false)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .compact()
        .init();
    
    let mut client = GreeterClient::connect("http://[::1]:3000").await?;
    loop {
        let request = tonic::Request::new(HelloRequest {
            name: format!("time: {:?}!", tokio::time::Instant::now()).into(),
        });
        info!("Sending request to gRPC Server...");
        let response = client.say_hello(request).await?;
    
        info!("RESPONSE={:?}", response);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    // Ok(())
}