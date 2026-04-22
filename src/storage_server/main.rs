mod storage;
use tonic::transport::Server;

use storage::{Storage, StorageServiceServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:31416".parse().unwrap();
    let svr = StorageServiceServer::new(Storage);
    Server::builder().add_service(svr).serve(addr).await?;
    Ok(())
}

/*
EJEMPLO BASICO DE CONEXIÓN SERVER

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let addr = "[::1]:21416".parse().unwrap();
    let svr = StorageServiceServer::new(Storage);

    Server::builder()
        .add_service(svr)
        .serve(addr)
        .await?;
    Ok(())
}

EJEMPLO BASICO DE CONEXIÓN CLIENTE

use client_storage_proto::storage_service_client::StorageServiceClient;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let endpoint = Endpoint::from_static();
    let mut connection = StorageServiceClient::connect(endpoint).await.unwrap();

    let request = AssignedNode {
        storage_node_id: "String".into(),
        ipv4_address: "String".into(),
        auth_token: "String".into(),
        file_id: "String".into(),
    };
    let response = connection.delete_file(request).await?;
    println!("respuesta: {:?}",response.into_inner());
}

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let channel = Endpoint::from_static("[::1]::31416").connect().await?;
    let mut connection = StorageServiceClient::new(channel);

    let request = AssignedNode {
        storage_node_id: "String".into(),
        ipv4_address: "String".into(),
        auth_token: "String".into(),
        file_id: "String".into(),
    };
    let response = connection.delete_file(request).await?;
    println!("respuesta: {:?}",response.into_inner());
}


EJEMPLO DE BALANCE DE CARGA
#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let ips = ["[::1]:21416","[::1]:21417"];

    let endpoints = ips.into_iter().map(|ip|Endpoint::from_static(ip));
    let channel = Channel::balance_list(endpoints);

    let mut connection = StorageServiceClient::new(channel);

    let request = AssignedNode {
        storage_node_id: "String".into(),
        ipv4_address: "String".into(),
        auth_token: "String".into(),
        file_id: "String".into(),
    };

    let response = connection.delete_file(request).await?;

    println!("respuesta: {:?}",response.into_inner());
    Ok(())
}
*/
