mod storage;

use storage::prelude::*;

use dotenvy::dotenv;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let addr = "[::1]:31416".parse().unwrap();

    let storage_svc = StorageServer::new(Storage);
    let metadata_instructions = MetadataInstructionsServer::new(Storage);

    Server::builder()
        .add_service(storage_svc)
        .add_service(metadata_instructions)
        .serve(addr)
        .await?;
    Ok(())
}
