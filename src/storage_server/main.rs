mod dist_fs_proto_buf {
    tonic::include_proto!("dist_fs");
}
use dist_fs_proto_buf::storage_service_server::{StorageService, StorageServiceServer};
use dist_fs_proto_buf::upload_chunk::Data;
use dist_fs_proto_buf::{AssignedNode, DownloadResponse, UploadChunk};

use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

use tonic::{Request, Response, Status, Streaming};

struct Storage;

#[derive(Clone)]
struct MyExtension{
    data: String
}

#[tonic::async_trait]
impl StorageService for Storage {
    async fn upload_file(
        &self,
        request: Request<Streaming<UploadChunk>>,
    ) -> Result<Response<()>, Status> {

        let mut streaming = request.into_inner();
        let header = match streaming.next().await {
            Some(chunk_result) => match chunk_result?.data {
                Some(Data::Header(h)) => h,
                _ => {
                    return Err(Status::invalid_argument("File Header must be sent first"));
                }
            },
            None => return Err(Status::invalid_argument("Empty stream")),
        };

        // TODO: SE DDEBE COMPROBAR QUE LA ACCIÓN ES LEGAL CON "AUTH TOKEN"

        // ! "TOTAL SIZE" PARECE SER INNUTIL, CONSIDERAR DESECHARLO (DEL HEADER)
        // ! "FILE NAME" SER INNUTIL, CONSIDERAR DESECHARLO (DEL HEADER)

        println!(
            "recibiendo archivo {} ({} bytes), sobreescribir: {}",
            header.file_name, header.total_size, header.overwrite
        );

        // TODO: El data server debe crear un UUID para nombrar el archivo, todavía falta
        // TODO: Después enviarle a metadata server el UUID con el que lo guardé

        // ! Todavía requiero una ruta alternativa cuando necesito "sobreescribir", eliminar el anterior o así está bien
        let mut file = File::create("nombre_generado_por_mi_xd")
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
        Ok(Response::new(()))
    }

    // !Me falta implementar este type, le puse "ReceiverStream" solo porque así está en el ejemplo pero no sé si es lo que necesito
    // !Dentro de wrappers existen otros tipos que implementan el trait Stream
    type DownloadFileStream = ReceiverStream<Result<DownloadResponse, Status>>;

    async fn download_file(
        &self,
        request: Request<AssignedNode>,
    ) -> Result<Response<Self::DownloadFileStream>, Status> {
        /*
        *NOTAS DE CAMBIOS!

        eliminar "total size" de UploadHeader
        eliminar "file name" de UploadHeader

        eliminar "node id" de AssignedNode
        eliminar "file id" de AssignedNode

        la función remota "download_file" no requiere "Assigned Node" como argumento
        Crear en su lugar un "DownloadRequest" que solo contenga "auth token"
        */
        // ! IPV4 PARECE SER INNUTIL, CONSIDERAR DESECHARLO (PARA ESTA REQUEST)
        // ? "FILE ID" PODRÍA SER INUTIL SI SE TOMA DE DESICIÓN DE DISEÑO DE OBTENER EL FILE ID DESDE EL METADATA SERVER
        // ! SE DEBE COMPROBAR QUE SE CONECTA AL NODO CORRECTO CON EL "NODE ID" O DESECHAR EL "NODE ID"
        // TODO: SE DDEBE COMPROBAR QUE LA ACCIÓN ES LEGAL CON "AUTH TOKEN"

        let (xs, xr) = mpsc::channel(10);
        let inner = request.into_inner();

        tokio::spawn(async move {
            // TODO: Aquí debería hacer un método para pedirle al metadata server que archivo quiero abrir

            let mut file = File::open(inner.file_id).await.unwrap();
            let mut buffer = [0_u8; 4096];

            let mut n: usize;
            loop {
                n = file.read(&mut buffer).await.unwrap();
                if n == 0 {
                    break;
                }

                let response = Ok(DownloadResponse {
                    content: buffer[..n].to_vec(),
                });

                if xs.send(response).await.is_err() {
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(xr)))
    }

    async fn delete_file(&self, request: Request<AssignedNode>) -> Result<Response<()>, Status> {
        // ! IPV4 PARECE SER INNUTIL, CONSIDERAR DESECHARLO (PARA ESTA REQUEST)
        // ? "FILE ID" PODRÍA SER INUTIL SI SE TOMA DE DESICIÓN DE DISEÑO DE OBTENER EL FILE ID DESDE EL METADATA SERVER
        // ! SE DEBE COMPROBAR QUE SE CONECTA AL NODO CORRECTO CON EL "NODE ID" O DESECHAR EL "NODE ID"
        // TODO: SE DDEBE COMPROBAR QUE LA ACCIÓN ES LEGAL CON "AUTH TOKEN"

        let assigned_node = request.into_inner();
        File::open(assigned_node.file_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(()))
    }
}

fn main() {}
