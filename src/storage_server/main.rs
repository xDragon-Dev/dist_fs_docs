use dotenvy::dotenv;
use tonic::transport::Server;

use doc_svc::storage::server::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let addr = "[::1]:31416".parse().unwrap();

    let storage_svc = StoragePrivateServer::new(Storage);
    let metadata_instructions = StorageInstructionsServer::new(Storage);

    Server::builder()
        .add_service(storage_svc)
        .add_service(metadata_instructions)
        .serve(addr)
        .await?;
    Ok(())
}
