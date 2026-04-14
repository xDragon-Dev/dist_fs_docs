mod dist_fs_proto_buf {
    tonic::include_proto!("client_storage");
}

use dist_fs_proto_buf::storage_service_server::{StorageService, StorageServiceServer};
use dist_fs_proto_buf::upload_chunk::Data;
use dist_fs_proto_buf::{DownloadResponse, FileRequest, UploadChunk};

use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

use tonic::transport::Server;
use tonic::{Request, Response, Status, Streaming};

use uuid::Uuid;

struct Storage;

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

        //Se comprueba aquí o en el middleware? si es en el middleware cómo?
        let jwt = header.auth_jwt;


        // TODO: SE DDEBE COMPROBAR QUE LA ACCIÓN ES LEGAL CON "AUTH TOKEN" (middleware en tower, cómo?)
        // TODO: Después enviarle a metadata server el UUID con el que lo guardé

        // ! Todavía requiero una ruta alternativa cuando necesito "sobreescribir", eliminar el anterior o así está bien
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

    // !Me falta implementar este type, le puse "ReceiverStream" solo porque así está en el ejemplo pero no sé si es lo que necesito
    // !Dentro de wrappers existen otros tipos que implementan el trait Stream
    type DownloadFileStream = ReceiverStream<Result<DownloadResponse, Status>>;

    async fn download_file(
        &self,
        request: Request<FileRequest>,
    ) -> Result<Response<Self::DownloadFileStream>, Status> {
        
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

    async fn delete_file(&self, request: Request<FileRequest>) -> Result<Response<()>, Status> {
        // ? "FILE ID" PODRÍA SER INUTIL SI SE TOMA DE DESICIÓN DE DISEÑO DE OBTENER EL FILE ID DESDE EL METADATA SERVER
        // TODO: SE DEBE COMPROBAR QUE SE CONECTA AL NODO CORRECTO CON EL "NODE ID" O DESECHAR EL "NODE ID"
        // TODO: SE DDEBE COMPROBAR QUE LA ACCIÓN ES LEGAL CON "AUTH TOKEN"

        let assigned_node = request.into_inner();

        tokio::fs::remove_file(assigned_node.file_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:21416".parse().unwrap();
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

use dist_fs_proto_buf::storage_service_client::StorageServiceClient;

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
