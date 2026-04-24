mod client_storage_proto {
    tonic::include_proto!("storage");
}

pub use client_storage_proto::private_storage_server::PrivateStorageServer;
pub use client_storage_proto::public_storage_server::PublicStorageServer;

use client_storage_proto::private_storage_server::PrivateStorage;
use client_storage_proto::public_storage_server::PublicStorage;

use client_storage_proto::upload_chunk::Data;
use client_storage_proto::{DownloadResponse, FileRequest, UploadChunk};

use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

use tonic::{Request, Response, Status, Streaming};

use uuid::Uuid;

pub struct Storage;

#[tonic::async_trait]
impl PublicStorage for Storage {
    type DownloadFileStream = ReceiverStream<Result<DownloadResponse, Status>>;

    async fn download_file(
        &self,
        request: Request<FileRequest>,
    ) -> Result<Response<Self::DownloadFileStream>, Status> {
        // ! SE DEBE COMPROBAR QUE SE CONECTA AL NODO CORRECTO CON EL "NODE ID" O DESECHAR EL "NODE ID"
        let (xs, xr) = mpsc::channel(10);
        let inner = request.into_inner();
        tokio::spawn(async move {
            let mut buffer = [0_u8; 4096];
            match File::open(inner.file_id).await {
                Ok(mut file) => {
                    while let Ok(n) = file.read(&mut buffer).await {
                        if n == 0 {
                            break;
                        }
                        let response = DownloadResponse {
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
}

#[tonic::async_trait]
impl PrivateStorage for Storage {
    async fn upload_file(
        &self,
        request: Request<Streaming<UploadChunk>>,
    ) -> Result<Response<()>, Status> {
        let mut streaming = request.into_inner();
        let checksum = match streaming.next().await {
            Some(chunk_result) => match chunk_result?.data {
                Some(Data::Header(h)) => h.checksum,
                _ => {
                    return Err(Status::invalid_argument("File Header must be sent first"));
                }
            },
            None => return Err(Status::invalid_argument("Empty stream")),
        };
        // TODO: Verificar que el checksum no existe ya en la base de datos, para no repetir
        println!("{checksum}");

        // TODO: Después enviarle a metadata server el UUID con el que lo guardé o que el metadata server lo genere y yo lo guardo aquí
        let uuid = Uuid::new_v4();

        let mut file = File::create(uuid.to_string())
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        while let Some(chunk_result) = streaming.next().await {
            match chunk_result?.data {
                Some(Data::Content(c)) => {
                    file.write_all(&c)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?;
                }
                _ => {
                    return Err(Status::invalid_argument("Expected file content"));
                }
            }
        }

        // ! Todavía requiero comprobar el checksum del archivo para comprobar que todo estaba bien

        Ok(Response::new(()))
    }

    async fn delete_file(&self, request: Request<FileRequest>) -> Result<Response<()>, Status> {
        // ? "FILE ID" PODRÍA SER INUTIL SI SE TOMA DE DESICIÓN DE DISEÑO DE OBTENER EL FILE ID DESDE EL METADATA SERVER
        // TODO: SE DEBE COMPROBAR QUE SE CONECTA AL NODO CORRECTO CON EL "NODE ID" O DESECHAR EL "NODE ID"

        let assigned_node = request.into_inner();

        tokio::fs::remove_file(assigned_node.file_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(()))
    }
}
