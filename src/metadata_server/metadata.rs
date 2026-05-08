mod metadata_proto {
    tonic::include_proto!("metadata");
}

mod storage_metadata_proto {
    tonic::include_proto!("storage_metadata");
}

use metadata_proto::private_metadata_server::PrivateMetadata;
pub use metadata_proto::private_metadata_server::PrivateMetadataServer;
use metadata_proto::public_metadata_server::PublicMetadata;
pub use metadata_proto::public_metadata_server::PublicMetadataServer;
use metadata_proto::{
    AssignedNode, ChangePasswordRequest, ChangeUserNameRequest, CreateSubTopicRequest,
    CreateTopicRequest, CreateUserRequest, DeleteUserRequest, DownloadNodeRequest, LogInRequest,
    ScientificDocument, ScientificDocumentRequest, SearchRequest, SearchResults, SubTopic,
    SubTopics, Topic, Topics, UploadNodeRequest,
};

//use storage_metadata_proto::storage_signals_server::StorageSignals;
//pub use storage_metadata_proto::storage_signals_server::StorageSignalsServer;

use storage_metadata_proto::metadata_instructions_client::MetadataInstructionsClient;
use storage_metadata_proto::DeleteFilesRequest;

use tonic::{Request, Response, Status};

use common::auth::*;
use common::types::{
    jwt_types::{self, *},
    sql_types::{self, *},
};
use uuid::Uuid;

use chrono::prelude::*;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use validator::Validate;

use crate::metadata::metadata_proto::{
    DeleteDocumentRequest, DeleteSubTopicRequest, DeleteTopicRequest, SearchResult,
};

pub struct Metadata {
    pub pg_pool: Pool<Postgres>,
}

#[tonic::async_trait]
impl PublicMetadata for Metadata {
    // * FUNCIÓN EN SU FORMA FINAL Y CORRECTA ✅
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<()>, Status> {
        request
            .get_ref()
            .validate()
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let desired_role =
            sql_types::Role::try_from(request.get_ref().user_role).map_err(|_| {
                Status::invalid_argument(format!(
                    "Invalid role value {}",
                    request.get_ref().user_role
                ))
            })?;

        let token_role = match request.metadata().get("jwt") {
            Some(metadata_jwt) => {
                let jwt = metadata_jwt
                    .to_str()
                    .map_err(|_| Status::invalid_argument(r#"Wrong "jwt" format"#))?;
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

        let role = if (token_role == None || token_role == Some(jwt_types::Role::User))
            && desired_role == sql_types::Role::Admin
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

        sqlx::query("INSERT INTO users (user_name, password_hash, user_role, tokens_valid_after) VALUES ($1, $2, $3, 0)")
            .bind(request.user_name)
            .bind(password_hash)
            .bind(role)
            .execute(&self.pg_pool)
            .await
            .map_err(|_| Status::already_exists("Username is already in use"))?;

        Ok(Response::new(()))
    }

    // * FUNCIÓN EN SU FORMA FINAL Y CORRECTA ✅
    async fn log_in(&self, request: Request<LogInRequest>) -> Result<Response<String>, Status> {
        let request = request.into_inner();

        let (role, password_hash): (sql_types::Role, String) =
            sqlx::query_as("SELECT user_role, password_hash FROM users WHERE user_name = $1")
                .bind(&request.user_name)
                .fetch_one(&self.pg_pool)
                .await
                .map_err(|_| Status::not_found("Nonexistent user"))?;

        verify_password_hash(request.password.as_bytes(), &password_hash)
            .map_err(|_| Status::permission_denied("Invalid user password"))?;

        let expiration_days = if request.remember { 30 } else { 1 };

        let now = Utc::now();
        let iat = now.timestamp();
        let exp = now
            .checked_add_days(chrono::Days::new(expiration_days))
            .ok_or(Status::internal("Date out of the 64-bit limit"))?
            .timestamp();

        let token_claims = TokenClaims {
            sub: request.user_name.clone(),
            user_role: role.into(),
            exp: exp,
            iat: iat,
        };

        let jwt =
            generate_jwt(token_claims).map_err(|_| Status::internal(r#""jwt"encoding failed"#))?;

        sqlx::query("UPDATE users SET tokens_valid_after = $1 WHERE user_name = $2")
            .bind(iat)
            .bind(request.user_name)
            .execute(&self.pg_pool)
            .await
            .map_err(|_| Status::not_found("Nonexistent user"))?;

        Ok(Response::new(jwt))
    }

    // * FUNCIÓN EN SU FORMA FINAL Y CORRECTA ✅
    async fn get_global_topics(&self, _request: Request<()>) -> Result<Response<Topics>, Status> {
        let topic_rows: Vec<(i32, String)> =
            sqlx::query_as("SELECT id, name FROM topics WHERE scope = 'Global'")
                .fetch_all(&self.pg_pool)
                .await
                .map_err(|_| Status::internal("Internal database error"))?;

        let topics = Topics {
            content: topic_rows
                .into_iter()
                .map(|(topic_id, topic_name)| Topic {
                    topic_id,
                    topic_name,
                })
                .collect(),
        };
        Ok(Response::new(topics))
    }

    // * FUNCIÓN EN SU FORMA FINAL Y CORRECTA ✅
    async fn get_global_sub_topics(
        &self,
        _request: Request<()>,
    ) -> Result<Response<SubTopics>, Status> {
        let sub_topic_rows: Vec<(i32, String)> =
            sqlx::query_as("SELECT id, name FROM sub_topics WHERE scope = 'Global'")
                .fetch_all(&self.pg_pool)
                .await
                .map_err(|_| Status::internal("Internal database error"))?;

        let sub_topics = SubTopics {
            content: sub_topic_rows
                .into_iter()
                .map(|(subtopic_id, subtopic_name)| SubTopic {
                    subtopic_id,
                    subtopic_name,
                })
                .collect(),
        };
        Ok(Response::new(sub_topics))
    }
}

/*
PARA CUALQUIER FUNCIÓŃ SE COMPROBARÁN LOS SIGUIENTES RUBROS GENERALES EN EL SIGUIENTE ORDEN
1. Comprobación y obtención de los JWT claims
2. Comprobación del usuario de ser si mismo o admin (Si es necesario)
3. Comprobar si el usuario a modificar


*/

#[tonic::async_trait]
impl PrivateMetadata for Metadata {
    // * FUNCIÓN EN SU FORMA FINAL Y CORRECTA ✅
    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<()>, Status> {
        let token_claims = request
            .extensions()
            .get::<TokenClaims>()
            .ok_or(Status::internal(
                r#"Authentication failure: Missing a required request extension "jwt_claims"#,
            ))?;
        if !(token_claims.sub == request.get_ref().user_name
            || token_claims.user_role == jwt_types::Role::Admin)
        {
            return Err(Status::permission_denied(
                "Insufficient permissions to do this operation",
            ));
        }
        let request = request.into_inner();

        let user_exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE user_name = $1)")
                .bind(request.user_name.clone())
                .fetch_one(&self.pg_pool)
                .await
                .map_err(|_| Status::internal("Internal database error"))?;

        if !user_exists {
            return Err(Status::not_found("Nonexistent user"));
        }

        let documents: Vec<(std::net::IpAddr, i32, Uuid)> = sqlx::query_as(
            "
            SELECT 
                sn.ip, 
                sn.port,
                d.id
            FROM users u
            JOIN scientific_documents d ON u.user_name = d.posted_by
            JOIN document_storage_nodes dsn ON d.id = dsn.document_id
            JOIN storage_nodes sn ON dsn.storage_node_id = sn.id
            WHERE u.user_name = $1
        ",
        )
        .bind(request.user_name.clone())
        .fetch_all(&self.pg_pool)
        .await
        .map_err(|_| Status::internal("Internal database error"))?;

        let documents = documents
            .into_iter()
            .map(|(sn_ip, sn_port, doc_id)| {
                let endpoint = match sn_ip {
                    IpAddr::V4(ipv4) => format!("https://{}:{}", ipv4, sn_port),
                    IpAddr::V6(ipv6) => format!("https://[{}]:{}", ipv6, sn_port),
                };
                (endpoint, doc_id.to_string())
            })
            .collect::<Vec<(String, String)>>();

        let mut documents_hash_map = HashMap::<String, Vec<String>>::new();
        documents.into_iter().for_each(|(endpoint, doc_id)| {
            documents_hash_map
                .entry(endpoint)
                .or_insert(Vec::new())
                .push(doc_id);
        });

        tokio::spawn(async move {
            for (endpoint, doc_ids) in documents_hash_map {
                let mut metadata_instructions =
                    MetadataInstructionsClient::connect(endpoint).await.unwrap();
                let request = DeleteFilesRequest { file_ids: doc_ids };
                metadata_instructions.delete_files(request).await.unwrap();
            }
        });

        sqlx::query("DELETE FROM scientific_documents WHERE posted_by = $1")
            .bind(request.user_name.clone())
            .execute(&self.pg_pool)
            .await
            .map_err(|_| Status::internal("Internal database error"))?;

        sqlx::query("DELETE FROM topics WHERE created_by = $1 AND scope = 'Local'")
            .bind(request.user_name.clone())
            .execute(&self.pg_pool)
            .await
            .map_err(|_| Status::internal("Internal database error"))?;

        sqlx::query("DELETE FROM sub_topics WHERE created_by = $1 AND scope = 'Local'")
            .bind(request.user_name.clone())
            .execute(&self.pg_pool)
            .await
            .map_err(|_| Status::internal("Internal database error"))?;

        sqlx::query("DELETE FROM users WHERE user_name = $1")
            .bind(request.user_name)
            .execute(&self.pg_pool)
            .await
            .map_err(|_| Status::internal("Internal database error"))?;

        Ok(Response::new(()))
    }

    // * FUNCIÓN EN SU FORMA FINAL Y CORRECTA ✅
    async fn change_password(
        &self,
        request: Request<ChangePasswordRequest>,
    ) -> Result<Response<()>, Status> {
        let token_claims = request
            .extensions()
            .get::<TokenClaims>()
            .ok_or(Status::internal(
                r#"Authentication failure: Missing a required request extension "jwt_claims"#,
            ))?;

        if !(token_claims.sub == request.get_ref().user_name
            || token_claims.user_role == jwt_types::Role::Admin)
        {
            return Err(Status::permission_denied(
                "Insufficient permissions to do this operation",
            ));
        }

        request
            .get_ref()
            .validate()
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let user_exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE user_name = $1)")
                .bind(request.get_ref().user_name.clone())
                .fetch_one(&self.pg_pool)
                .await
                .map_err(|_| Status::internal("Internal database error"))?;
        if !user_exists {
            return Err(Status::not_found("Nonexistent user"));
        }

        if token_claims.user_role == jwt_types::Role::User {
            let password_hash: String =
                sqlx::query_scalar("SELECT password_hash FROM users WHERE user_name = $1")
                    .bind(request.get_ref().user_name.clone())
                    .fetch_one(&self.pg_pool)
                    .await
                    .map_err(|e| Status::aborted(e.to_string()))?;

            verify_password_hash(
                request.get_ref().current_password.as_bytes(),
                &password_hash,
            )
            .map_err(|_| Status::permission_denied("Invalid user password"))?;
        }

        let request = request.into_inner();

        let password_hash = hash_password(request.new_password.as_bytes())
            .map_err(|_| Status::internal("Password hashing failed"))?;

        let now = Utc::now().timestamp();

        sqlx::query(
            "UPDATE users SET (password_hash, tokens_valid_after) = ($1, $2) WHERE user_name = $3",
        )
        .bind(password_hash)
        .bind(now)
        .bind(request.user_name)
        .execute(&self.pg_pool)
        .await
        .map_err(|_| Status::internal("Internal database error"))?;

        Ok(Response::new(()))
    }

    // * FUNCIÓN EN SU FORMA FINAL Y CORRECTA ✅
    async fn change_user_name(
        &self,
        request: Request<ChangeUserNameRequest>,
    ) -> Result<Response<()>, Status> {
        let token_claims = request
            .extensions()
            .get::<TokenClaims>()
            .ok_or(Status::internal(
                r#"Authentication failure: Missing a required request extension "jwt_claims"#,
            ))?;

        if !(token_claims.sub == request.get_ref().current_user_name
            || token_claims.user_role == jwt_types::Role::Admin)
        {
            return Err(Status::permission_denied(
                "Insufficient permissions to do this operation",
            ));
        }

        let request = request.into_inner();

        request
            .validate()
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let user_exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE user_name = $1)")
                .bind(request.current_user_name.clone())
                .fetch_one(&self.pg_pool)
                .await
                .map_err(|_| Status::internal("Internal database error"))?;
        if !user_exists {
            return Err(Status::not_found("Nonexistent user"));
        }

        let now = Utc::now().timestamp();

        sqlx::query(
            "UPDATE users SET (user_name, tokens_valid_after) = ($1, $2) WHERE user_name = $3",
        )
        .bind(request.new_user_name)
        .bind(now)
        .bind(request.current_user_name)
        .execute(&self.pg_pool)
        .await
        .map_err(|_| Status::already_exists("Username is already in use"))?;

        Ok(Response::new(()))
    }

    // ! ⚠️ NO CONSIDERA QUE EXISTEN VARIOS NODOS PARA ELEGIR 1 ⚠️
    // ? NO IMPLEMENTADO AÚN
    async fn get_download_node(
        &self,
        request: Request<DownloadNodeRequest>,
    ) -> Result<Response<AssignedNode>, Status> {
        let request = request.into_inner();
        let (ip, port): (std::net::IpAddr, i32) = sqlx::query_as(
            "SELECT sn.ip sn.port
            FROM storage_nodes sn
            JOIN document_storage_nodes dsn
            ON dsn.storage_node_id = sn.id
            WHERE dsn.document_id = $1
        ",
        )
        .bind(request.document_id)
        .fetch_one(&self.pg_pool)
        .await
        .map_err(|e| Status::aborted(e.to_string()))?;

        let response: AssignedNode = AssignedNode {
            ipv4_address: ip.to_string(),
            port: port.to_string(),
        };
        Ok(Response::new(response))
    }

    // ? NO IMPLEMENTADO AÚN
    async fn get_upload_node(
        &self,
        _request: Request<UploadNodeRequest>,
    ) -> Result<Response<AssignedNode>, Status> {
        Err(Status::aborted("aborted"))
    }

    // ? NO IMPLEMENTADO AÚN
    async fn delete_document(
        &self,
        _request: Request<DeleteDocumentRequest>,
    ) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    // * FUNCION EN SU FORMA FINAL Y CORRECTA ✅
    async fn create_topic(
        &self,
        request: Request<CreateTopicRequest>,
    ) -> Result<Response<()>, Status> {
        let token_claims = request
            .extensions()
            .get::<TokenClaims>()
            .ok_or(Status::internal(
                r#"Authentication failure: Missing a required request extension "jwt_claims"#,
            ))?;

        let scope = sql_types::Scope::try_from(request.get_ref().scope).map_err(|_| {
            Status::invalid_argument(format!("Invalid scope value {}", request.get_ref().scope))
        })?;

        if scope == sql_types::Scope::Global && token_claims.user_role == jwt_types::Role::User {
            return Err(Status::permission_denied(
                "Insufficient permissions to do this operation",
            ));
        }

        sqlx::query("INSERT INTO topics (name, created_by, scope) VALUES ($1, $2, $3)")
            .bind(request.get_ref().name.clone())
            .bind(token_claims.sub.clone())
            .bind(scope)
            .execute(&self.pg_pool)
            .await
            .map_err(|_| Status::already_exists("Topic already exists"))?;
        Ok(Response::new(()))
    }

    // * FUNCION EN SU FORMA FINAL Y CORRECTA ✅
    async fn delete_topic(
        &self,
        request: Request<DeleteTopicRequest>,
    ) -> Result<Response<()>, Status> {
        let token_claims = request
            .extensions()
            .get::<TokenClaims>()
            .ok_or(Status::internal(
                r#"Authentication failure: Missing a required request extension "jwt_claims"#,
            ))?;

        let created_by: Option<String> =
            sqlx::query_scalar("SELECT created_by FROM topics WHERE id = $1")
                .bind(request.get_ref().topic_id)
                .fetch_one(&self.pg_pool)
                .await
                .map_err(|_| Status::not_found("Nonexistent topic"))?;

        if !(Some(&token_claims.sub) == created_by.as_ref()
            || token_claims.user_role == jwt_types::Role::Admin)
        {
            return Err(Status::permission_denied(
                "Insufficient permissions to do this operation",
            ));
        }

        let request = request.into_inner();
        sqlx::query("DELETE FROM topics WHERE id = $1")
            .bind(request.topic_id)
            .execute(&self.pg_pool)
            .await
            .map_err(|_| Status::not_found("Nonexistent topic"))?;
        Ok(Response::new(()))
    }

    // * FUNCION EN SU FORMA FINAL Y CORRECTA ✅
    async fn get_local_topics(&self, request: Request<()>) -> Result<Response<Topics>, Status> {
        let token_claims = request
            .extensions()
            .get::<TokenClaims>()
            .ok_or(Status::internal(
                r#"Authentication failure: Missing a required request extension "jwt_claims"#,
            ))?;

        let topics: Vec<(i32, String)> =
            sqlx::query_as("SELECT id, name FROM topics WHERE created_by = $1 AND scope = 'Local'")
                .bind(token_claims.sub.clone())
                .fetch_all(&self.pg_pool)
                .await
                .map_err(|_| Status::internal("Internal database error"))?;

        let topics = Topics {
            content: topics
                .into_iter()
                .map(|(topic_id, topic_name)| Topic {
                    topic_id,
                    topic_name,
                })
                .collect(),
        };

        Ok(Response::new(topics))
    }

    // * FUNCION EN SU FORMA FINAL Y CORRECTA ✅
    async fn create_sub_topic(
        &self,
        request: Request<CreateSubTopicRequest>,
    ) -> Result<Response<()>, Status> {
        let token_claims = request
            .extensions()
            .get::<TokenClaims>()
            .ok_or(Status::internal(
                r#"Authentication failure: Missing a required request extension "jwt_claims"#,
            ))?;

        let scope = sql_types::Scope::try_from(request.get_ref().scope).map_err(|_| {
            Status::invalid_argument(format!("Invalid scope value {}", request.get_ref().scope))
        })?;

        if scope == sql_types::Scope::Global && token_claims.user_role == jwt_types::Role::User {
            return Err(Status::permission_denied(
                "Insufficient permissions to do this operation",
            ));
        }

        sqlx::query("INSERT INTO sub_topics (name, created_by, scope) VALUES ($1, $2, $3)")
            .bind(request.get_ref().name.clone())
            .bind(token_claims.sub.clone())
            .bind(scope)
            .execute(&self.pg_pool)
            .await
            .map_err(|_| Status::already_exists("Subtopic already exists"))?;
        Ok(Response::new(()))
    }

    // * FUNCION EN SU FORMA FINAL Y CORRECTA ✅
    async fn delete_sub_topic(
        &self,
        request: Request<DeleteSubTopicRequest>,
    ) -> Result<Response<()>, Status> {
        let token_claims = request
            .extensions()
            .get::<TokenClaims>()
            .ok_or(Status::internal(
                r#"Authentication failure: Missing a required request extension "jwt_claims"#,
            ))?;

        let created_by: Option<String> =
            sqlx::query_scalar("SELECT created_by FROM sub_topics WHERE id = $1")
                .bind(request.get_ref().subtopic_id)
                .fetch_one(&self.pg_pool)
                .await
                .map_err(|_| Status::not_found("Nonexistent subtopic"))?;

        if !(Some(&token_claims.sub) == created_by.as_ref()
            || token_claims.user_role == jwt_types::Role::Admin)
        {
            return Err(Status::permission_denied(
                "Insufficient permissions to do this operation",
            ));
        }

        let request = request.into_inner();
        sqlx::query("DELETE FROM sub_topics WHERE id = $1")
            .bind(request.subtopic_id)
            .execute(&self.pg_pool)
            .await
            .map_err(|_| Status::not_found("Nonexistent subtopic"))?;
        Ok(Response::new(()))
    }

    // * FUNCION EN SU FORMA FINAL Y CORRECTA ✅
    async fn get_local_sub_topics(
        &self,
        request: Request<()>,
    ) -> Result<Response<SubTopics>, Status> {
        let token_claims = request
            .extensions()
            .get::<TokenClaims>()
            .ok_or(Status::internal(
                r#"Authentication failure: Missing a required request extension "jwt_claims"#,
            ))?;

        let subtopics: Vec<(i32, String)> = sqlx::query_as(
            "SELECT id, name FROM sub_topics WHERE created_by = $1 AND scope = 'Local'",
        )
        .bind(token_claims.sub.clone())
        .fetch_all(&self.pg_pool)
        .await
        .map_err(|_| Status::internal("Internal database error"))?;

        let subtopics = SubTopics {
            content: subtopics
                .into_iter()
                .map(|(subtopic_id, subtopic_name)| SubTopic {
                    subtopic_id,
                    subtopic_name,
                })
                .collect(),
        };

        Ok(Response::new(subtopics))
    }

    // * FUNCION EN SU FORMA FINAL Y CORRECTA ✅
    async fn search_documents(
        &self,
        request: Request<SearchRequest>,
    ) -> Result<Response<SearchResults>, Status> {
        let claims = request
            .extensions()
            .get::<TokenClaims>()
            .ok_or(Status::unauthenticated(
                r#"Authentication failure: Missing a required request extension "jwt_claims"#,
            ))?
            .to_owned();

        let is_admin = claims.user_role == jwt_types::Role::Admin;

        let req = request.into_inner();

        // Convertimos los tipos de gRPC a tipos compatibles con SQLx/Postgres
        let mut document_types: Vec<sql_types::DocumentType> = Vec::new();
        for t in req.document_types.into_iter() {
            let r#type = sql_types::DocumentType::try_from(t)
                .map_err(|_| Status::invalid_argument(format!("Invalid scope value {}", t)))?;
            document_types.push(r#type);
        }

        let start_date = DateTime::<Utc>::from_timestamp_secs(req.start_date);
        let end_date = DateTime::<Utc>::from_timestamp_secs(req.end_date);

        let content_pattern = if !req.content.is_empty() {
            Some(format!("%{}%", req.content))
        } else {
            None
        };

        // Ejecutamos una query estática. Postgres ignorará los filtros donde el parámetro sea NULL o la lista esté vacía.
        let rows: Vec<SearchResultRow> = sqlx::query_as(
            r#"
            SELECT DISTINCT d.* 
            FROM scientific_documents d
            LEFT JOIN document_topics dt ON d.id = dt.document_id
            LEFT JOIN topics t ON dt.topic_id = t.id
            LEFT JOIN document_sub_topics dst ON d.id = dst.document_id
            LEFT JOIN sub_topics st ON dst.sub_topic_id = st.id
            WHERE 
                ($1 OR d.posted_by = $2)
                -- Filtros de contenido (SearchKind)
                AND (
                    $3 IS NULL OR (
                        ($4 = 0 AND d.title ILIKE $3) OR
                        ($4 = 1 AND d.posted_by ILIKE $3) OR
                        ($4 = 2 AND $5 = ANY(d.authors)) OR
                        ($4 = 3 AND $5 = ANY(d.keywords))
                    )
                )
                AND (cardinality($6) = 0 OR t.id = ANY($6))
                AND (cardinality($7) = 0 OR st.id = ANY($7))
                AND (cardinality($8) = 0 OR d.document_type = ANY($8))
                AND (cardinality($9) = 0 OR d.language = ANY($9))

                AND ($10 IS NULL OR d.publication_date >= $10)
                AND ($11 IS NULL OR d.publication_date <= $11)
            "#,
        )
        .bind(is_admin)
        .bind(claims.sub.clone())
        .bind(content_pattern)
        .bind(req.search_kind)
        .bind(req.content)
        .bind(&req.topics)
        .bind(&req.sub_topics)
        .bind(&document_types)
        .bind(&req.languages)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pg_pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let results = rows
            .into_iter()
            .map(|r| SearchResult {
                id: r.id.to_string(),
                posted_by: r.posted_by,
                title: r.title,
                document_type: r.document_type.into(),
                publication_date: r.publication_date.timestamp(),
                language: r.language,
            })
            .collect();

        Ok(Response::new(SearchResults { results }))
    }

    // * FUNCIÓN EN SU FORMA FINAL Y CORRECTA ✅
    async fn get_scientific_document(
        &self,
        request: Request<ScientificDocumentRequest>,
    ) -> Result<Response<ScientificDocument>, Status> {
        let token_claims = request
            .extensions()
            .get::<TokenClaims>()
            .ok_or(Status::internal(
                r#"Authentication failure: Missing a required request extension "jwt_claims"#,
            ))?;

        let id = uuid::Uuid::from_str(&request.get_ref().document_id)
            .map_err(|_| Status::invalid_argument("Invalid id format"))?;

        let posted_by: String =
            sqlx::query_scalar("SELECT posted_by FROM scientific_documents WHERE id = $1")
                .bind(id)
                .fetch_one(&self.pg_pool)
                .await
                .map_err(|e| {
                    println!("{e}");
                    Status::aborted("Nonexistent document")
                })?;

        if !(token_claims.sub == posted_by || token_claims.user_role == jwt_types::Role::Admin) {
            return Err(Status::permission_denied(
                "Insufficient permissions to do this operation",
            ));
        }

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
}
