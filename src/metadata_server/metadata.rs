mod client_metadata_proto {
    tonic::include_proto!("metadata");
}

pub use client_metadata_proto::private_metadata_server::PrivateMetadataServer;
pub use client_metadata_proto::public_metadata_server::PublicMetadataServer;

use client_metadata_proto::private_metadata_server::PrivateMetadata;
use client_metadata_proto::public_metadata_server::PublicMetadata;

use client_metadata_proto::*;
use tonic::{Request, Response, Status};

use common::auth::*;
use common::types::TokenClaims;

use chrono::prelude::*;
use sqlx::{Pool, Postgres};
use validator::Validate;

pub struct Metadata {
    pub pg_pool: Pool<Postgres>,
}

#[tonic::async_trait]
impl PublicMetadata for Metadata {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<()>, Status> {
        use common::types::Role;
        request
            .get_ref()
            .validate()
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let desired_role = Role::try_from(request.get_ref().user_role).map_err(|_| {
            Status::invalid_argument(format!(
                "Invalid role value {}",
                request.get_ref().user_role
            ))
        })?;

        let token_role = match request.metadata().get("jwt") {
            Some(metadata_jwt) => {
                let jwt = metadata_jwt
                    .to_str()
                    .map_err(|e| Status::invalid_argument(e.to_string()))?;
                let token_claims = verify_jwt::<TokenClaims>(jwt).map_err(|e| match e.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        Status::permission_denied("Expired token")
                    }
                    _ => Status::invalid_argument("Token decodification failed"),
                })?;
                Some(token_claims.user_role)
            }
            None => None,
        };

        let role = if (token_role == None || token_role == Some(Role::User))
            && desired_role == Role::Admin
        {
            return Err(Status::permission_denied(
                "Insufficient permissions to do this operation",
            ));
        } else {
            desired_role
        };

        let request = request.into_inner();
        let password_hash = hash_password(request.password.as_bytes())
            .map_err(|_| Status::internal("Password hashing failed"))?;

        sqlx::query("INSERT INTO users (user_name, password_hash, user_role) VALUES ($1, $2, $3)")
            .bind(request.user_name)
            .bind(password_hash)
            .bind(role)
            .execute(&self.pg_pool)
            .await
            .map_err(|e| Status::aborted(e.to_string()))?;

        Ok(Response::new(()))
    }

    async fn log_in(&self, request: Request<LogInRequest>) -> Result<Response<String>, Status> {
        let request = request.into_inner();

        let (role, password_hash): (common::types::Role, String) =
            sqlx::query_as("SELECT user_role, password_hash FROM users WHERE user_name = $1")
                .bind(&request.user_name)
                .fetch_one(&self.pg_pool)
                .await
                .map_err(|e| Status::aborted(e.to_string()))?;

        verify_password_hash(request.password.as_bytes(), &password_hash)
            .map_err(|_| Status::permission_denied("Invalid user password"))?;

        let expiration_days = if request.remember { 30 } else { 1 };

        let exp = Utc::now()
            .checked_add_days(chrono::Days::new(expiration_days))
            .ok_or(Status::internal("Date out of the 64-bit limit"))?
            .timestamp();

        let token_claims = TokenClaims {
            sub: request.user_name,
            user_role: role,
            exp: exp,
        };
        let jwt = generate_jwt(token_claims).map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(jwt))
    }

    // ! FUNCIÓN DIFICIL DE IMPLEMENTAR, DEJADO PARA EL FINAL
    async fn get_download_node(
        &self,
        request: Request<NodeRequest>,
    ) -> Result<Response<AssignedNode>, Status> {
        Err(Status::aborted("aborted"))
    }

    async fn get_topics(&self, _request: Request<()>) -> Result<Response<Topics>, Status> {
        let topics: Vec<Topic> = sqlx::query_as("SELECT name FROM topics")
            .fetch_all(&self.pg_pool)
            .await
            .map_err(|e| Status::aborted(e.to_string()))?;
        let topics = Topics { content: topics };
        Ok(Response::new(topics))
    }

    async fn get_sub_topics(&self, _request: Request<()>) -> Result<Response<SubTopics>, Status> {
        let sub_topics: Vec<SubTopic> = sqlx::query_as("SELECT name FROM sub_topics")
            .fetch_all(&self.pg_pool)
            .await
            .map_err(|e| Status::aborted(e.to_string()))?;
        let sub_topics = SubTopics {
            content: sub_topics,
        };
        Ok(Response::new(sub_topics))
    }

    async fn get_scientific_document(
        &self,
        request: Request<ScientificDocumentRequest>,
    ) -> Result<Response<ScientificDocument>, Status> {
        let request = request.into_inner();

        struct DocumentRow {
            pub posted_by: String,
            pub title: String,
            pub authors: Vec<String>,
            pub r#abstract: String,
            pub keywords: Vec<String>,
            pub document_type: i32,
            pub publication_date: i64,
            pub language: String,
        }

        
        /*
        let file: ScientificDocument = sqlx::query_as("").bind(request.id)
            .fetch_one(&self.pg_pool)
            .await
            .map_err(|e| Status::aborted(e.to_string()))?;
        */

        
        Err(Status::aborted("aborted"))
    }

    /*
        CREATE TABLE scientific_documents (
        id UUID PRIMARY KEY, --DEFAULT gen_random_uuid()
        posted_by TEXT NOT NULL REFERENCES users(user_name),
        title TEXT NOT NULL,
        authors TEXT[] NOT NULL,
        abstract TEXT NOT NULL,
        keywords TEXT[] NOT NULL,
        document_type document_type NOT NULL,
        publication_date TIMESTAMPTZ NOT NULL, --DEFAULT CURRENT_TIMESTAMP
        language TEXT NOT NULL
        );
     */

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
