mod auth;
mod validation;
mod client_storage_proto {
    tonic::include_proto!("client_storage");
}

use std::path::Path;

use futures_util::StreamExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_util::io::ReaderStream;

use client_storage_proto::UploadChunk;
use client_storage_proto::UploadHeader;
use client_storage_proto::storage_service_client::StorageServiceClient;
use client_storage_proto::upload_chunk::Data;

use crate::client_storage_proto::FileRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Loads .env file and all of its variables
    upload_file("Filename.pdf").await?;
    download_file().await?;
    delete_file().await?;
    Ok(())
}

async fn upload_file(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = StorageServiceClient::connect("http://[::1]:31416").await?;
    let header_chunk = UploadChunk {
        data: Some(Data::Header(UploadHeader {
            auth_jwt: "JWT".into(),
            operation_id: "UUID".into(),
            overwrite: true, //No sirve XDDD
            checksum: "Checksum".into(),
        })),
    };
    let file = File::open(path).await?;
    let reader_stream = ReaderStream::new(file);

    let content_stream = reader_stream.map(|result| match result {
        Ok(bytes) => UploadChunk {
            data: Some(Data::Content(bytes.to_vec())),
        },
        Err(_) => UploadChunk { data: None },
    });
    let chunks = tokio_stream::once(header_chunk).chain(content_stream);
    connection.upload_file(chunks).await?;
    Ok(())
}

async fn download_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = StorageServiceClient::connect("http://[::1]:31416").await?;
    let request = FileRequest {
        auth_jwt: "JWT".into(),
        operation_id: "UUID".into(),
        storage_node_id: "UUID".into(),
        file_id: "e030747e-f2b9-4bd1-a464-1e6c8541dae4".into(),
    };
    let response = connection.download_file(request).await?;
    let mut stream = response.into_inner();
    let mut file = File::create("El nombre yo lo pongo").await?;
    while let Some(result) = stream.next().await {
        file.write(&result?.content).await?;
    }
    Ok(())
}

async fn delete_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = StorageServiceClient::connect("http://[::1]:31416").await?;
    let request = FileRequest {
        auth_jwt: "JWT".into(),
        operation_id: "UUID".into(),
        storage_node_id: "UUID".into(),
        file_id: "e030747e-f2b9-4bd1-a464-1e6c8541dae4".into(),
    };
    connection.delete_file(request).await?;
    Ok(())
}

/*
EJEMPLO BASICO DE CONEXIÓN SERVER

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let addr = "[::1]:31416".parse().unwrap();
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
