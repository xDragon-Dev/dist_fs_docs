mod metadata_proto {
    tonic::include_proto!("metadata");
}

use std::str::FromStr;

pub use metadata_proto::private_metadata_server::PrivateMetadataServer;
pub use metadata_proto::public_metadata_server::PublicMetadataServer;

use metadata_proto::private_metadata_server::PrivateMetadata;
use metadata_proto::public_metadata_server::PublicMetadata;

use metadata_proto::{
    AssignedNode,
    CreateUserRequest,
    DeleteUserRequest,
    LogInRequest,
    NodeRequest,
    NodeUploadRequest,
    ScientificDocument,
    ScientificDocumentRequest,
    SearchRequest,
    //SearchResult,
    SearchResults,
    SubTopics,
    Topics,
};

use tonic::{Request, Response, Status};

use common::auth::*;
use common::types::{jwt_types::*, sql_types::*};

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
        request
            .get_ref()
            .validate()
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let desired_role = TokenRole::try_from(request.get_ref().user_role).map_err(|_| {
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

        let role = if (token_role == None || token_role == Some(TokenRole::User))
            && desired_role == TokenRole::Admin
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

        let (role, password_hash): (TokenRole, String) =
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
        let topic_rows: Vec<String> = sqlx::query_scalar("SELECT name FROM topics")
            .fetch_all(&self.pg_pool)
            .await
            .map_err(|e| Status::aborted(e.to_string()))?;

        let topics = Topics {
            content: topic_rows
        };
        Ok(Response::new(topics))
    }

    async fn get_sub_topics(&self, _request: Request<()>) -> Result<Response<SubTopics>, Status> {
        let sub_topic_rows: Vec<String> = sqlx::query_scalar("SELECT name FROM sub_topics")
            .fetch_all(&self.pg_pool)
            .await
            .map_err(|e| Status::aborted(e.to_string()))?;

        let sub_topics = SubTopics {
            content: sub_topic_rows
        };
        Ok(Response::new(sub_topics))
    }

    async fn get_scientific_document(
        &self,
        request: Request<ScientificDocumentRequest>,
    ) -> Result<Response<ScientificDocument>, Status> {
        let request = request.into_inner();
        let id = uuid::Uuid::from_str(&request.id)
            .map_err(|_| Status::invalid_argument("Invalid id format"))?;
        let scientific_document_row: ScientificDocumentRow = sqlx::query_as(
            "
            SELECT 
                d.posted_by, 
                d.title, 
                d.authors, 
                d.abstract,
                d.keywords,
                COALESCE((
                    SELECT jsonb_agg(t.name)
                    FROM topics t
                    JOIN document_topics dt ON dt.topic_id = t.id
                    WHERE dt.document_id = d.id
                ), '[]') as topics,
                COALESCE((
                    SELECT jsonb_agg(st.name)
                    FROM sub_topics st
                    JOIN document_sub_topics dst ON dst.sub_topic_id = st.id
                    WHERE dst.document_id = d.id
                ), '[]') as sub_topics,
                d.document_type,
                d.publication_date, 
                d.language
            FROM scientific_documents d
            WHERE d.id = $1;
        ",
        )
        .bind(id)
        .fetch_one(&self.pg_pool)
        .await
        .map_err(|e| Status::aborted(e.to_string()))?;

        let scientific_document = ScientificDocument {
            posted_by: scientific_document_row.posted_by,
            title: scientific_document_row.title,
            authors: scientific_document_row.authors,
            r#abstract: scientific_document_row.r#abstract,
            keywords: scientific_document_row.keywords,
            topics: scientific_document_row.topics.0,
            sub_topics: scientific_document_row.sub_topics.0,
            document_type: scientific_document_row.document_type.into(),
            publication_date: scientific_document_row.publication_date.timestamp(),
            language: scientific_document_row.language,
        };
        Ok(Response::new(scientific_document))
    }

    async fn search_documents(
        &self,
        request: Request<SearchRequest>,
    ) -> Result<Response<SearchResults>, Status> {
        /*

        pub enum SearchKind {
            Title,
            Publisher,
            Author,
            Keywod
        }

        pub struct SearchRequest {
            pub content: String,
            pub search_kind: SearchKind,
            pub topics: Vec<String>,
            pub sub_topics: Vec<String>,
            pub document_types: Vec<i32>,
            pub start_date: i64,
            pub end_date: i64,
            pub languages: Vec String>,
        }

        pub struct SearchResult {
            pub id: String,
            pub posted_by: String,
            pub title: String,
            pub document_type: i32,
            pub publication_date: i64,
            pub language: String,
        }
        */

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

    async fn create_topic(&self, request: Request<String>) -> Result<Response<()>, Status> {
        Err(Status::aborted("aborted"))
    }

    async fn delete_topic(&self, request: Request<String>) -> Result<Response<()>, Status> {
        Err(Status::aborted("aborted"))
    }

    async fn create_sub_topic(&self, request: Request<String>) -> Result<Response<()>, Status> {
        Err(Status::aborted("aborted"))
    }

    async fn delete_sub_topic(&self, request: Request<String>) -> Result<Response<()>, Status> {
        Err(Status::aborted("aborted"))
    }
}
