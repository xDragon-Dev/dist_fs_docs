use crate::common::authentication::verify_jwt;
use crate::common::types::jwt_types::Claims;

use tonic::Status;

use sqlx::PgPool;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tonic::metadata::MetadataMap;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct AuthLayer {
    pub db: PgPool,
}

impl AuthLayer {
    pub fn new(db: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { db }
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        AuthMiddleware {
            inner: service,
            db: self.db.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
    db: PgPool,
}

type BoxFuture<'a, T> = Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

impl<S, ReqBody, ResBody> Service<http::Request<ReqBody>> for AuthMiddleware<S>
where
    S: Service<http::Request<ReqBody>, Response = http::Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Default + Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: http::Request<ReqBody>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let db = self.db.clone();

        Box::pin(async move {
            let metadata = MetadataMap::from_headers(req.headers().clone());

            match auth_logic(metadata, &db).await {
                Ok(claims) => {
                    let mut req = req;
                    req.extensions_mut().insert(claims);
                    inner.call(req).await
                }
                Err(status) => Ok(status.into_http::<ResBody>()),
            }
        })
    }
}

async fn auth_logic(metadata: MetadataMap, db: &PgPool) -> Result<Claims, Status> {
    let jwt = metadata
        .get("jwt")
        .ok_or_else(|| Status::unauthenticated(r#"No "jwt" was provided"#))?
        .to_str()
        .map_err(|_| Status::invalid_argument(r#"Wrong "jwt" format"#))?;

    let claims = verify_jwt::<Claims>(jwt).map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
            Status::permission_denied("Expired token")
        }
        _ => Status::invalid_argument("Token decodification failed"),
    })?;

    let tokens_valid_after: i64 =
        sqlx::query_scalar("SELECT tokens_valid_after FROM users WHERE name = $1")
            .bind(&claims.sub)
            .fetch_one(db)
            .await
            .map_err(|_| Status::not_found("Token subject not found in database"))?;

    if tokens_valid_after > claims.iat {
        return Err(Status::permission_denied(
            "The given token is no longer valid",
        ));
    }

    Ok(claims)
}
