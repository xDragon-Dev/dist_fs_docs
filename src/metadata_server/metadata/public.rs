mod metadata_proto {
    tonic::include_proto!("metadata");
}

use metadata_proto::public_metadata_server::PublicMetadata;
pub use metadata_proto::public_metadata_server::PublicMetadataServer;

use metadata_proto::{CreateUserRequest, LogInRequest, SubTopic, SubTopics, Topic, Topics};

use tonic::{Request, Response, Status};

use common::auth::*;
use common::types::{
    jwt_types::{self, *},
    sql_types,
};

use chrono::prelude::*;
use validator::Validate;

#[tonic::async_trait]
impl PublicMetadata for super::Metadata {
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
                let token_claims = verify_jwt::<Claims>(jwt).map_err(|e| match e.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        Status::permission_denied("Expired token")
                    }
                    _ => Status::invalid_argument("Token decodification failed"),
                })?;
                Some(token_claims.role)
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

        sqlx::query("INSERT INTO users (name, password_hash, role, tokens_valid_after) VALUES ($1, $2, $3, 0)")
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
            sqlx::query_as("SELECT role, password_hash FROM users WHERE name = $1")
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

        let token_claims = Claims {
            sub: request.user_name.clone(),
            role: role.into(),
            exp: exp,
            iat: iat,
        };

        let jwt =
            generate_jwt(token_claims).map_err(|_| Status::internal(r#""jwt"encoding failed"#))?;

        sqlx::query("UPDATE users SET tokens_valid_after = $1 WHERE name = $2")
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
