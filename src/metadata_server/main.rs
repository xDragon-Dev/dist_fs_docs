mod database;
mod metadata;

use dotenvy::dotenv;
use std::env;

use tonic::service::{InterceptorLayer, LayerExt};
use tonic::transport::Server;
use tower::ServiceBuilder;

use common::middleware::{PermisionLayer, auth_jwt};
use metadata::{Metadata, PrivateMetadataServer, PublicMetadataServer};

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

    let layer = ServiceBuilder::new()
        .layer(InterceptorLayer::new(auth_jwt))
        .layer(PermisionLayer);

    let priv_svc = layer.named_layer(PrivateMetadataServer::new(Metadata {
        pg_pool: pool.clone(),
    }));
    let pub_svc = PublicMetadataServer::new(Metadata {
        pg_pool: pool.clone(),
    });

    Server::builder()
        .add_service(priv_svc)
        .add_service(pub_svc)
        .serve(addr)
        .await?;

    Ok(())
}
