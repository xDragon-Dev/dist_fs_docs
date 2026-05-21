use tonic::service::LayerExt;
use tonic::transport::Server;
use tower::ServiceBuilder;

use doc_svc::metadata::server::*;

use std::env;
use dotenvy::dotenv;

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("Enviroment variable \"DATABASE_URL\" must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let addr = "[::1]:31415".parse().unwrap();

    let layer = ServiceBuilder::new().layer(AuthLayer::new(pool.clone()));

    let priv_svc = MetadataPrivateServer::new(Metadata::new(pool.clone()));
    let pub_svc = MetadataPublicServer::new(Metadata::new(pool.clone()));

    let priv_svc = layer.named_layer(priv_svc);

    Server::builder()
        .add_service(priv_svc)
        .add_service(pub_svc)
        .serve(addr)
        .await?;

    Ok(())
}
