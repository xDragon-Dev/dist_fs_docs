mod checksum;
mod client_storage_proto {
    tonic::include_proto!("storage");
}

use futures_util::StreamExt;
use std::path::Path;

use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_util::io::ReaderStream;

use client_storage_proto::private_storage_client::PrivateStorageClient;
use client_storage_proto::public_storage_client::PublicStorageClient;
use client_storage_proto::{FileRequest, UploadChunk, UploadHeader, upload_chunk::Data};

use common::auth::generate_jwt;
use common::types::{Role, TokenClaims};

use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    upload_file("File_name.pdf").await?;
    download_file().await?;
    delete_file().await?;
    Ok(())
}

async fn upload_file(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = PrivateStorageClient::connect("http://[::1]:31416").await?;
    let header_chunk = UploadChunk {
        data: Some(Data::Header(UploadHeader {
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

    let jwt_claims = TokenClaims {
        sub: "Juanito".into(),
        user_role: Role::User,
        exp: i64::MAX,
    };

    let jwt = generate_jwt(jwt_claims).unwrap();

    let chunks = tokio_stream::once(header_chunk).chain(content_stream);
    let mut request = tonic::Request::new(chunks);
    let headder_map = request.metadata_mut();
    headder_map.insert("jwt", jwt.parse().unwrap());
    headder_map.insert(
        "operation_id",
        "Jsjsjs este si no sirve de nadota".parse().unwrap(),
    );
    connection.upload_file(request).await?;
    Ok(())
}

async fn download_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = PublicStorageClient::connect("http://[::1]:31416").await?;
    let request = FileRequest {
        file_id: "79f8dacd-84be-49f8-addd-09977c28666b".into(),
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
    let mut connection = PrivateStorageClient::connect("http://[::1]:31416").await?;
    let file_request = FileRequest {
        file_id: "79f8dacd-84be-49f8-addd-09977c28666b".into(),
    };

    let mut request = tonic::Request::new(file_request);

    let jwt_claims = TokenClaims {
        sub: "Juanito".into(),
        user_role: Role::User,
        exp: i64::MAX,
    };

    let jwt = generate_jwt(jwt_claims).unwrap();

    let headder_map = request.metadata_mut();
    headder_map.insert("jwt", jwt.parse().unwrap());
    headder_map.insert(
        "operation_id",
        "Jsjsjs este si no sirve de nadota".parse().unwrap(),
    );
    connection.delete_file(request).await?;
    Ok(())
}
