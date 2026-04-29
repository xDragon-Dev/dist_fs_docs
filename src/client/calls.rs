mod client_storage_proto {
    tonic::include_proto!("storage");
}

use std::collections::HashMap;
use std::path::Path;

use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

use client_storage_proto::private_storage_client::PrivateStorageClient;
use client_storage_proto::public_storage_client::PublicStorageClient;
use client_storage_proto::{FileRequest, UploadChunk, UploadFooter, upload_chunk::Data};

use common::auth::generate_jwt;
use common::types::{Role, TokenClaims};

use sha2::{Digest, Sha256};

async fn _upload_file(
    path: impl AsRef<Path> + 'static + Send,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = PrivateStorageClient::connect("http://[::1]:31416").await?;
    let (xs, xr) = mpsc::channel(10);

    // ! ⚠️ Este es de las pocas partes del proyecto que puede TEORICAMENTE causar un PANIC! por los unwrap
    // ! 💀 El hilo podría no vivir lo suficiente como para completar su proposito (🚧🚧🚧 IN TEST 🚧🚧🚧)
    tokio::spawn(async move {
        let mut buffer = [0_u8; 65536];
        let mut hasher = Sha256::new();
        if let Ok(mut file) = File::open(path).await {
            while let Ok(n) = file.read(&mut buffer).await {
                if n == 0 {
                    break;
                }
                xs.send(UploadChunk {
                    data: Some(Data::Content(buffer.to_vec())),
                })
                .await
                .unwrap();
                hasher.update(buffer);
            }
            let checksum = hasher
                .finalize()
                .iter()
                .map(|bytes| format!("{:02x}", bytes))
                .collect::<String>();
            xs.send(UploadChunk {
                data: Some(Data::Footer(UploadFooter { checksum })),
            })
            .await
            .unwrap();
        }
    });
    let mut request = tonic::Request::new(ReceiverStream::new(xr));

    let jwt_claims = TokenClaims {
        sub: "Juanito".into(),
        user_role: Role::User,
        exp: i64::MAX,
    };
    let header = HashMap::from([
        // !Posible crasheo teorico
        ("jwt", generate_jwt(jwt_claims).unwrap()),
        ("operation_id", uuid::Uuid::new_v4().to_string()),
    ]);

    let metadata_map = request.metadata_mut();
    header.iter().for_each(|(key, val)| {
        // !Posible crasheo teorico
        metadata_map.insert(*key, val.parse().unwrap());
    });

    connection.upload_file(request).await?;
    Ok(())
}

async fn _download_file() -> Result<(), Box<dyn std::error::Error>> {
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

async fn _delete_file() -> Result<(), Box<dyn std::error::Error>> {
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

    let header = HashMap::from([
        // !Posible crasheo teorico
        ("jwt", generate_jwt(jwt_claims).unwrap()),
        (
            "operation_id",
            String::from("79f8dacd-84be-49f8-addd-09977c28666b"),
        ),
    ]);

    let metadata = request.metadata_mut();

    header.iter().for_each(|(key, val)| {
        // !Posible crasheo teorico
        metadata.insert(*key, val.parse().unwrap());
    });
    connection.delete_file(request).await?;
    Ok(())
}
