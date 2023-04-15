use greeter::greeter_client::GreeterClient;
use greeter::HelloRequest;

// Import the generated proto-rust file into a module
pub mod greeter {
    tonic::include_proto!("greeter");
}