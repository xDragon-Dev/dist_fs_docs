mod storage_private_proto {
    tonic::include_proto!("storage_private");
}

use storage_private_proto::storage_private_server::StoragePrivate;
pub use storage_private_proto::storage_private_server::StoragePrivateServer;

use storage_private_proto::upload_chunk::Data;
use storage_private_proto::*;

use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

use tonic::{Request, Response, Status, Streaming};

use sha2::{Digest, Sha256};
use uuid::Uuid;

#[tonic::async_trait]
impl StoragePrivate for super::Storage {
    type DownloadFileStream = ReceiverStream<Result<DownloadChunk, Status>>;
    /*
    ? NECESITA AUTENTICACIÓN: SI
    Por qué? Suponiendo que el metadata server ya hizo su trabajo dandote el nodo correcto para el id de un archivo que
    ya corroboró que es tuyo, pero eso no garantiza que alguien quiera descargar cualquier archivo teniendo la identidad
    que tenga

    Se requiere el uso de jwt, se envía a un metadata server y que corrobore todas las credednciales están bien
    */
    async fn download_file(
        &self,
        request: Request<DownloadFileRequest>,
    ) -> Result<Response<Self::DownloadFileStream>, Status> {
        let (xs, xr) = mpsc::channel(10);
        let file_request = request.into_inner();
        tokio::spawn(async move {
            let mut buffer = [0_u8; 65536];
            match File::open(file_request.file_id).await {
                Ok(mut file) => {
                    while let Ok(n) = file.read(&mut buffer).await {
                        if n == 0 {
                            break;
                        }
                        let response = DownloadChunk {
                            content: buffer[..n].to_vec(),
                        };
                        if xs.send(Ok(response)).await.is_err() {
                            break;
                        }
                    }
                }
                Err(e) => {
                    let _ = xs.send(Err(Status::not_found(e.to_string()))).await;
                }
            }
        });
        Ok(Response::new(ReceiverStream::new(xr)))
    }

    async fn upload_file(
        &self,
        request: Request<Streaming<UploadChunk>>,
    ) -> Result<Response<()>, Status> {
        let mut streaming = request.into_inner();
        // TODO: Después enviarle a metadata server el UUID con el que lo guardé o que el metadata server lo genere y yo lo guardo aquí
        let uuid = Uuid::new_v4();

        let mut file = File::create(uuid.to_string())
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let mut hasher = Sha256::new();

        while let Some(chunk_result) = streaming.next().await {
            match chunk_result?.data {
                Some(data) => match data {
                    Data::Content(c) => {
                        file.write_all(&c)
                            .await
                            .map_err(|e| Status::internal(e.to_string()))?;
                        hasher.update(c);
                    }
                    Data::Footer(f) => {
                        let checksum = hasher
                            .finalize()
                            .iter()
                            .map(|bytes| format!("{:02x}", bytes))
                            .collect::<String>();
                        if checksum == f.checksum {
                            break;
                        } else {
                            return Err(Status::data_loss("Unrecoverable data loss or corruption"));
                        }
                    }
                },
                None => {
                    return Err(Status::invalid_argument("Empty stream"));
                }
            }
        }
        Ok(Response::new(()))
    }
}
