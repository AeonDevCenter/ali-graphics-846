// lib.rs
pub mod error;
pub mod handlers;
pub mod routes;
pub mod types;

// main functions
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;

pub async fn get_client_config() -> Client {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    println!("AWS Config loaded: {:?}", config.region());
    Client::new(&config)
}
