mod storage;

use storage::{MetadataInstructionsServer, Storage, StorageServiceServer};

use dotenvy::dotenv;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let addr = "[::1]:31416".parse().unwrap();

    let storage_svc = StorageServiceServer::new(Storage);
    let metadata_instructions = MetadataInstructionsServer::new(Storage);

    Server::builder()
        .add_service(storage_svc)
        .add_service(metadata_instructions)
        .serve(addr)
        .await?;
    Ok(())
}
