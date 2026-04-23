mod client_metadata_proto {
    tonic::include_proto!("metadata");
}

pub use client_metadata_proto::private_metadata_server::PrivateMetadataServer;
pub use client_metadata_proto::public_metadata_server::PublicMetadataServer;

use client_metadata_proto::private_metadata_server::PrivateMetadata;
use client_metadata_proto::public_metadata_server::PublicMetadata;

use client_metadata_proto::*;
use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};

pub struct Metadata {
    pub pg_pool: Pool<Postgres>,
}

#[tonic::async_trait]
impl PublicMetadata for Metadata {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<()>, Status> {
        let user_request = request.into_inner();
        sqlx::query("INSERT INTO users (user_name, password_hash, user_role) VALUE ($1,$2,'User')")
            .bind(user_request.user_name)
            .bind(user_request.password)
            .execute(&self.pg_pool)
            .await
            .map_err(|e| Status::already_exists(e.to_string()))?;

        Ok(Response::new(()))
    }

    async fn log_in(&self, request: Request<LogInRequest>) -> Result<Response<String>, Status> {
        Err(Status::aborted("aborted"))
    }

    async fn get_download_node(
        &self,
        request: Request<NodeRequest>,
    ) -> Result<Response<AssignedNode>, Status> {
        Err(Status::aborted("aborted"))
    }

    async fn get_topics(&self, request: Request<()>) -> Result<Response<Topics>, Status> {
        Err(Status::aborted("aborted"))
    }

    async fn get_sub_topics(&self, request: Request<()>) -> Result<Response<Topics>, Status> {
        Err(Status::aborted("aborted"))
    }

    async fn get_scientific_document(
        &self,
        request: Request<ScientificDocumentRequest>,
    ) -> Result<Response<ScientificDocument>, Status> {
        Err(Status::aborted("aborted"))
    }

    async fn search_documents(
        &self,
        request: Request<SearchRequest>,
    ) -> Result<Response<SearchResults>, Status> {
        Err(Status::aborted("aborted"))
    }
}

#[tonic::async_trait]
impl PrivateMetadata for Metadata {
    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::aborted("aborted"))
    }

    async fn get_upload_node(
        &self,
        request: Request<NodeUploadRequest>,
    ) -> Result<Response<AssignedNode>, Status> {
        Err(Status::aborted("aborted"))
    }

    async fn get_delete_node(
        &self,
        request: Request<NodeRequest>,
    ) -> Result<Response<AssignedNode>, Status> {
        Err(Status::aborted("aborted"))
    }

    async fn create_topic(&self, request: Request<Topic>) -> Result<Response<()>, Status> {
        Err(Status::aborted("aborted"))
    }

    async fn delete_topic(&self, request: Request<Topic>) -> Result<Response<()>, Status> {
        Err(Status::aborted("aborted"))
    }

    async fn create_sub_topic(&self, request: Request<SubTopic>) -> Result<Response<()>, Status> {
        Err(Status::aborted("aborted"))
    }

    async fn delete_sub_topic(&self, request: Request<SubTopic>) -> Result<Response<()>, Status> {
        Err(Status::aborted("aborted"))
    }
}
