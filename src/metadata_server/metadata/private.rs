mod metadata_private_proto {
    tonic::include_proto!("metadata_private");
}

mod storage_instructions_proto {
    tonic::include_proto!("storage_instructions");
}

use metadata_private_proto::metadata_private_server::MetadataPrivate;
pub use metadata_private_proto::metadata_private_server::MetadataPrivateServer;

use metadata_private_proto::{
    AssignedNode, ChangePasswordRequest, ChangeUserNameRequest, CreateSubTopicRequest,
    CreateTopicRequest, DeleteUserRequest, DownloadNodeRequest, ScientificDocument,
    ScientificDocumentRequest, SearchRequest, SearchResults, SubTopic, SubTopics, Topic, Topics,
    UploadNodeRequest,
};

use storage_instructions_proto::storage_instructions_client::StorageInstructionsClient;
use storage_instructions_proto::{DeleteFileRequest, DeleteFilesRequest};
use tonic::{Request, Response, Status};

use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use common::auth::*;
use common::types::{
    jwt_types::{self, *},
    sql_types::{self, *},
};
use uuid::Uuid;
use validator::Validate;

use metadata_private_proto::{
    DeleteDocumentRequest, DeleteSubTopicRequest, DeleteTopicRequest, SearchResult,
};

#[tonic::async_trait]
impl MetadataPrivate for super::Metadata {
    // * FUNCIÓN CASI CORRECTA 😭
    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<()>, Status> {
        let token_claims = request
            .extensions()
            .get::<Claims>()
            .ok_or(Status::internal(
                r#"Authentication failure: Missing a required request extension "jwt_claims"#,
            ))?;
        if !(token_claims.sub == request.get_ref().user_name
            || token_claims.role == jwt_types::Role::Admin)
        {
            return Err(Status::permission_denied(
                "Insufficient permissions to do this operation",
            ));
        }
        let request = request.into_inner();

        let user_exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE name = $1)")
                .bind(&request.user_name)
                .fetch_one(&self.pg_pool)
                .await
                .map_err(|_| Status::internal("Internal database error"))?;

        if !user_exists {
            return Err(Status::not_found("Nonexistent user"));
        }

        // * RESCATE DE INFORMACIÓN ANTES DE LA ELIMINACIÓN
        let document_locations: Vec<(Uuid, std::net::IpAddr, i32, Uuid)> = sqlx::query_as(
            "
            SELECT 
                sn.id
                sn.ip, 
                sn.port,
                d.id
            FROM users u
            JOIN scientific_documents d ON u.name = d.posted_by
            JOIN document_storage_nodes dsn ON d.id = dsn.document_id
            JOIN storage_nodes sn ON dsn.storage_node_id = sn.id
            WHERE u.name = $1
        ",
        )
        .bind(&request.user_name)
        .fetch_all(&self.pg_pool)
        .await
        .map_err(|_| Status::internal("Internal database error"))?;

        // * ELIMINACIÓN DE METADATOS RELEVANTES AL USUARIO A ELIMINAR (INCLUYENDO PROPIAMENTE AL USUARIO)

        let queries = [
            "DELETE FROM scientific_documents WHERE posted_by = $1",
            "DELETE FROM topics WHERE created_by = $1 AND scope = 'Local'",
            "DELETE FROM sub_topics WHERE created_by = $1 AND scope = 'Local'",
            "DELETE FROM users WHERE name = $1",
        ];

        let mut tx = self.pg_pool.begin().await.map_err(|_| {
            Status::internal("Internal database error, unable to start transaction")
        })?;

        for query in queries {
            sqlx::query(query)
                .bind(&request.user_name)
                .execute(&mut *tx)
                .await
                .map_err(|_| Status::internal("Internal database error"))?;
        }

        tx.commit()
            .await
            .map_err(|_| Status::internal("Transaction commit failed"))?;

        // * SEGUNDA PARTE ELIMINACIÓN DE LOS ARCHIVOS EN LOS NODOS DE ALMACENAMIENTO

        let document_locations = document_locations
            .into_iter()
            .map(|(sn_id, sn_ip, sn_port, doc_id)| {
                let endpoint = match sn_ip {
                    IpAddr::V4(ipv4) => format!("https://{}:{}", ipv4, sn_port),
                    IpAddr::V6(ipv6) => format!("https://[{}]:{}", ipv6, sn_port),
                };
                (sn_id, endpoint, doc_id)
            })
            .collect::<Vec<(Uuid, String, Uuid)>>();

        let mut documents_hash_map = HashMap::<(Uuid, String), Vec<Uuid>>::new();
        document_locations
            .into_iter()
            .for_each(|(sn_id, endpoint, doc_id)| {
                documents_hash_map
                    .entry((sn_id, endpoint))
                    .or_insert(Vec::new())
                    .push(doc_id);
            });

        let pool = self.pg_pool.clone();
        tokio::spawn(async move {
            for (endpoint, doc_ids) in documents_hash_map {
                let mut metadata_instructions =
                    match StorageInstructionsClient::connect(endpoint.1.clone()).await {
                        Ok(client) => client,
                        Err(_) => {
                            sqlx::query(
                            "INSERT INTO failed_deletions(storage_node_id, files) VALUES ($1, $2)",
                        )
                        .bind(endpoint.0)
                        .bind(doc_ids)
                        .execute(&pool)
                        .await
                        // ! ⚠️ TEORICAMENTE ESTO NO PUEDE FALLAR SI ESTÁ BIEN CONFIGURADA LA DATABASE ⚠️
                        // ! 💀💀💀 PERO SI ESTO FALLA ESTO... SOLO QUEDA LLAMAR A DIOS Y REPARAR MANUALMENTE 💀💀💀
                        .ok();
                            continue;
                        }
                    };
                let request = DeleteFilesRequest {
                    file_ids: doc_ids.into_iter().map(|id| id.to_string()).collect(),
                };
                // ! 🚧 SI ALGO MALO PUEDE PASAR, PASARÁ AQUÍ PERO SERÁ IGNORADO 🚧
                metadata_instructions.delete_files(request).await.ok();
            }
        });

        Ok(Response::new(()))
    }

    // * FUNCIÓN EN SU FORMA FINAL Y CORRECTA ✅
    async fn change_password(
        &self,
        request: Request<ChangePasswordRequest>,
    ) -> Result<Response<()>, Status> {
        let token_claims = request
            .extensions()
            .get::<Claims>()
            .ok_or(Status::internal(
                r#"Authentication failure: Missing a required request extension "jwt_claims"#,
            ))?;

        if !(token_claims.sub == request.get_ref().user_name
            || token_claims.role == jwt_types::Role::Admin)
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
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE name = $1)")
                .bind(&request.get_ref().user_name)
                .fetch_one(&self.pg_pool)
                .await
                .map_err(|_| Status::internal("Internal database error"))?;
        if !user_exists {
            return Err(Status::not_found("Nonexistent user"));
        }

        if token_claims.role == jwt_types::Role::User {
            let password_hash: String =
                sqlx::query_scalar("SELECT password_hash FROM users WHERE name = $1")
                    .bind(&request.get_ref().user_name)
                    .fetch_one(&self.pg_pool)
                    .await
                    .map_err(|_| Status::internal("Internal database error"))?;

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
            "UPDATE users SET (password_hash, tokens_valid_after) = ($1, $2) WHERE name = $3",
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
            .get::<Claims>()
            .ok_or(Status::internal(
                r#"Authentication failure: Missing a required request extension "jwt_claims"#,
            ))?;

        if !(token_claims.sub == request.get_ref().current_user_name
            || token_claims.role == jwt_types::Role::Admin)
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
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE name = $1)")
                .bind(request.current_user_name.clone())
                .fetch_one(&self.pg_pool)
                .await
                .map_err(|_| Status::internal("Internal database error"))?;
        if !user_exists {
            return Err(Status::not_found("Nonexistent user"));
        }

        let now = Utc::now().timestamp();

        sqlx::query("UPDATE users SET (name, tokens_valid_after) = ($1, $2) WHERE name = $3")
            .bind(request.new_user_name)
            .bind(now)
            .bind(request.current_user_name)
            .execute(&self.pg_pool)
            .await
            .map_err(|_| Status::already_exists("Username is already in use"))?;

        Ok(Response::new(()))
    }

    // * FUNCION EN SU FORMA FINAL Y CORRECTA ✅
    // TODO: iMPLEMENTACIÓN DE IDENTIFICACIÓN DE OPERACIÓN
    async fn get_download_node(
        &self,
        request: Request<DownloadNodeRequest>,
    ) -> Result<Response<AssignedNode>, Status> {
        let token_claims = request
            .extensions()
            .get::<Claims>()
            .ok_or(Status::internal(
                r#"Authentication failure: Missing a required request extension "jwt_claims"#,
            ))?;

        let id = uuid::Uuid::from_str(&request.get_ref().document_id)
            .map_err(|_| Status::invalid_argument("Invalid id format"))?;

        let owner: String = sqlx::query_scalar(
            "SELECT u.name
            FROM users u
            JOIN scientific_documents d ON d.posted_by = u.name
            WHERE d.id = $1
        ",
        )
        .bind(&id)
        .fetch_one(&self.pg_pool)
        .await
        .map_err(|_| Status::not_found("Nonexistent document"))?;

        if !(token_claims.sub == owner || token_claims.role == jwt_types::Role::Admin) {
            return Err(Status::permission_denied(
                "Insufficient permissions to do this operation",
            ));
        }

        let (ip, port): (std::net::IpAddr, i32) = sqlx::query_as(
            "SELECT sn.ip, sn.port
            FROM storage_nodes sn
            JOIN document_storage_nodes dsn
            ON dsn.storage_node_id = sn.id
            WHERE dsn.document_id = $1
        ",
        )
        .bind(id)
        .fetch_one(&self.pg_pool)
        .await
        .map_err(|_| Status::not_found("Nonexistent document"))?;

        let end_point = match ip {
            IpAddr::V4(ipv4) => format!("https://{}:{}", ipv4, port),
            IpAddr::V6(ipv6) => format!("https://[{}]:{}", ipv6, port),
        };

        // ! ⚠️⚠️ POR IMPLEMENTAR ⚠️⚠️
        let response: AssignedNode = AssignedNode {
            end_point,
            operation: String::new(),
        };
        Ok(Response::new(response))
    }

    // ? NO IMPLEMENTADO AÚN
    // TODO: LA IMPLEMENTACIÓN DE BALACE DE CARGA
    // TODO: iMPLEMENTACIÓN DE IDENTIFICACIÓN DE OPERACIÓN
    async fn get_upload_node(
        &self,
        _request: Request<UploadNodeRequest>,
    ) -> Result<Response<AssignedNode>, Status> {
        Err(Status::aborted("aborted"))
    }

    // * FUNCION CASI CORRECTA 😭
    async fn delete_document(
        &self,
        request: Request<DeleteDocumentRequest>,
    ) -> Result<Response<()>, Status> {
        let token_claims = request
            .extensions()
            .get::<Claims>()
            .ok_or(Status::internal(
                r#"Authentication failure: Missing a required request extension "jwt_claims"#,
            ))?;

        let id = uuid::Uuid::from_str(&request.get_ref().document_id)
            .map_err(|_| Status::invalid_argument("Invalid id format"))?;

        let owner: String = sqlx::query_scalar(
            "SELECT u.name
            FROM users u
            JOIN scientific_documents d ON d.posted_by = u.name
            WHERE d.id = $1
        ",
        )
        .bind(&id)
        .fetch_one(&self.pg_pool)
        .await
        .map_err(|_| Status::not_found("Nonexistent document"))?;

        if !(token_claims.sub == owner || token_claims.role == jwt_types::Role::Admin) {
            return Err(Status::permission_denied(
                "Insufficient permissions to do this operation",
            ));
        }

        // * RESCATE DE INFORMACIÓN ANTES DE LA ELIMINACIÓN
        let (sn_id, sn_ip, sn_port): (Uuid, std::net::IpAddr, i32) = sqlx::query_as(
            "SELECT sn.id, sn.ip, sn.port
            FROM storage_nodes sn
            JOIN document_storage_nodes dsn
            ON dsn.storage_node_id = sn.id
            WHERE dsn.document_id = $1
        ",
        )
        .bind(&id)
        .fetch_one(&self.pg_pool)
        .await
        .map_err(|_| Status::not_found("Nonexistent document"))?;

        // * ELIMINACIÓN DEL DOCUMENTO DE LA BASE DE DATOS
        sqlx::query("DELETE FROM scientific_documents WHERE id = $1")
            .bind(&id)
            .execute(&self.pg_pool)
            .await
            .map_err(|_| Status::not_found("Nonexistent document"))?;

        let end_point = match sn_ip {
            IpAddr::V4(ipv4) => format!("https://{}:{}", ipv4, sn_port),
            IpAddr::V6(ipv6) => format!("https://[{}]:{}", ipv6, sn_port),
        };

        // * ELIMINACIÓN DEL ARCHIVOS EN NODOS DE ALMACENAMIENTO
        let mut connection = match StorageInstructionsClient::connect(end_point).await {
            Ok(conn) => conn,
            Err(_) => {
                sqlx::query("INSERT INTO failed_deletions(storage_node_id, files) VALUES ($1, $2)")
                    .bind(sn_id)
                    .bind(vec![id])
                    .execute(&self.pg_pool)
                    .await
                    // ! ⚠️ TEORICAMENTE ESTO NO PUEDE FALLAR SI ESTÁ BIEN CONFIGURADA LA DATABASE ⚠️
                    // ! 💀💀💀 PERO SI ESTO FALLA ESTO... SOLO QUEDA LLAMAR A DIOS Y REPARAR MANUALMENTE 💀💀💀
                    .ok();

                // * RETORNAR UN 'OK' EN LUGAR DE 'ERR' ES UNA DESICIÓN DE TRANSPARENCIA PARA EVITAR
                // * QUE EL CLIENTE TENGA QUE ENTERARSE DE MALCOMUNICACIÓN INTERNA
                return Ok(Response::new(()));
            }
        };

        let request = DeleteFileRequest {
            file_id: id.to_string(),
        };
        // ! 🚧 SI ALGO MALO PUEDE PASAR, PASARÁ AQUÍ PERO SERÁ IGNORADO 🚧
        connection.delete_file(request).await.ok();

        Ok(Response::new(()))
    }

    // * FUNCION EN SU FORMA FINAL Y CORRECTA ✅
    async fn create_topic(
        &self,
        request: Request<CreateTopicRequest>,
    ) -> Result<Response<()>, Status> {
        let token_claims = request
            .extensions()
            .get::<Claims>()
            .ok_or(Status::internal(
                r#"Authentication failure: Missing a required request extension "jwt_claims"#,
            ))?;

        let scope = sql_types::Scope::try_from(request.get_ref().scope).map_err(|_| {
            Status::invalid_argument(format!("Invalid scope value {}", request.get_ref().scope))
        })?;

        if scope == sql_types::Scope::Global && token_claims.role == jwt_types::Role::User {
            return Err(Status::permission_denied(
                "Insufficient permissions to do this operation",
            ));
        }

        sqlx::query("INSERT INTO topics (name, created_by, scope) VALUES ($1, $2, $3)")
            .bind(&request.get_ref().name)
            .bind(&token_claims.sub)
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
            .get::<Claims>()
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
            || token_claims.role == jwt_types::Role::Admin)
        {
            return Err(Status::permission_denied(
                "Insufficient permissions to do this operation",
            ));
        }

        let has_relations: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM document_topics WHERE topic_id = $1)")
                .bind(request.get_ref().topic_id)
                .fetch_one(&self.pg_pool)
                .await
                .map_err(|_| Status::not_found("Nonexistent topic"))?;

        if has_relations && token_claims.role == jwt_types::Role::User {
            return Err(Status::aborted("Unable to delete a document related topic"));
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
            .get::<Claims>()
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
            .get::<Claims>()
            .ok_or(Status::internal(
                r#"Authentication failure: Missing a required request extension "jwt_claims"#,
            ))?;

        let scope = sql_types::Scope::try_from(request.get_ref().scope).map_err(|_| {
            Status::invalid_argument(format!("Invalid scope value {}", request.get_ref().scope))
        })?;

        if scope == sql_types::Scope::Global && token_claims.role == jwt_types::Role::User {
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
            .get::<Claims>()
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
            || token_claims.role == jwt_types::Role::Admin)
        {
            return Err(Status::permission_denied(
                "Insufficient permissions to do this operation",
            ));
        }

        let has_relations: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM document_sub_topics WHERE sub_topic_id = $1)",
        )
        .bind(request.get_ref().subtopic_id)
        .fetch_one(&self.pg_pool)
        .await
        .map_err(|_| Status::not_found("Nonexistent subtopic"))?;

        if has_relations && token_claims.role == jwt_types::Role::User {
            return Err(Status::aborted(
                "Unable to delete a document related subtopic",
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
            .get::<Claims>()
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
            .get::<Claims>()
            .ok_or(Status::unauthenticated(
                r#"Authentication failure: Missing a required request extension "jwt_claims"#,
            ))?
            .to_owned();

        let is_admin = claims.role == jwt_types::Role::Admin;

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
            .get::<Claims>()
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

        if !(token_claims.sub == posted_by || token_claims.role == jwt_types::Role::Admin) {
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
