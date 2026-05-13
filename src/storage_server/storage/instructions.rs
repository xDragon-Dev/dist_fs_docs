mod storage_instructions_proto {
    tonic::include_proto!("storage_instructions");
}

use storage_instructions_proto::storage_instructions_server::StorageInstructions;
pub use storage_instructions_proto::storage_instructions_server::StorageInstructionsServer;
use storage_instructions_proto::{DeleteFileRequest, DeleteFilesRequest};

use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl StorageInstructions for super::Storage {
    async fn delete_file(
        &self,
        request: Request<DeleteFileRequest>,
    ) -> Result<Response<()>, Status> {
        let request = request.into_inner();
        tokio::fs::remove_file(request.file_id)
            .await
            .map_err(|_| Status::not_found("File not found"))?;
        Ok(Response::new(()))
    }

    async fn delete_files(
        &self,
        request: Request<DeleteFilesRequest>,
    ) -> Result<Response<()>, Status> {
        let request = request.into_inner();
        let mut messages = Vec::new();
        for file in request.file_ids {
            if let Err(_) = tokio::fs::remove_file(&file).await {
                let err_message = format!("Failed deleting file {}", file);
                messages.push(err_message);
                continue;
            }
        }
        if !messages.is_empty() {
            return Err(Status::data_loss(messages.join("\n")));
        }
        Ok(Response::new(()))
    }
}
