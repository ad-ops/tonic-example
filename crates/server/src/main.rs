use tonic::transport::Server;
use tracing::info;

use crate::server::MyGreeter;
use crate::server::greeter::greeter_server::GreeterServer;
use color_eyre::eyre::Result;

mod server;

// Use the tokio runtime to run our server
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
    let addr = "0.0.0.0:3000".parse()?;
    let greeter = MyGreeter::default();

    info!("Starting gRPC Server on 0.0.0.0:3000...");
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}