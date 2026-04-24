mod storage;
mod middleware;
use middleware::{MyMiddlewareLayer, auth_jwt};
use storage::{PrivateStorageServer, PublicStorageServer, Storage};

use dotenvy::dotenv;

use tonic::service::{InterceptorLayer, LayerExt};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let addr = "[::1]:31416".parse().unwrap();

    let layer = tower::ServiceBuilder::new()
        .layer(InterceptorLayer::new(auth_jwt))
        .layer(MyMiddlewareLayer);
    
    let pub_svc = PublicStorageServer::new(Storage);
    let priv_svc = layer.named_layer(PrivateStorageServer::new(Storage));

    Server::builder()
        .add_service(pub_svc)
        .add_service(priv_svc)
        .serve(addr)
        .await?;
    Ok(())
}
