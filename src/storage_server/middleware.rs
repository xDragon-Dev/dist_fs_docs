mod client_storage_proto {
    tonic::include_proto!("storage");
}

use std::{
    env,
    pin::Pin,
    task::{Context, Poll},
};

use tonic::{Request, Status};
use tower::{Layer, Service};

use jsonwebtoken::decode;
use jsonwebtoken::{DecodingKey, Validation};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
enum Role {
    Admin,
    User,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct JWTClaims {
    pub sub: String,
    pub user_role: Role,
    pub exp: i64,
}

// An interceptor function.
pub fn auth_jwt(mut req: Request<()>) -> Result<Request<()>, Status> {
    let header_jwt = req
        .metadata()
        .get("jwt")
        .and_then(|result| result.to_str().ok());
    match header_jwt {
        Some(jwt) => {
            let claims = verificate_jwt(jwt).map_err(|e| Status::unauthenticated(e.to_string()))?;
            req.extensions_mut().insert(claims);
        }
        None => {
            return Err(Status::unauthenticated(r#"No "jwt" was provided"#));
        }
    }

    Ok(req)
}

// ! Esto está directamente replicado, tratar de agregar a una librería común
fn verificate_jwt(jwt: &str) -> Result<JWTClaims, jsonwebtoken::errors::Error> {
    let secret = env::var("SECRET").expect("Enviroment variable \"SECRET\" must be set");
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

    let jwt_claims = decode::<JWTClaims>(&jwt, &decoding_key, &validation)?;
    Ok(jwt_claims.claims)
}

#[derive(Debug, Clone)]
pub struct MyMiddlewareLayer;

impl<S> Layer<S> for MyMiddlewareLayer {
    type Service = MyMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        MyMiddleware { inner: service }
    }
}

#[derive(Debug, Clone)]
pub struct MyMiddleware<S> {
    inner: S,
}

type BoxFuture<'a, T> = Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

impl<S, ReqBody, ResBody> Service<http::Request<ReqBody>> for MyMiddleware<S>
where
    S: Service<http::Request<ReqBody>, Response = http::Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Default,
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

        Box::pin(async move {
            // 1. Extraer el operation_id de los headers (Metadata)
            let op_id_header = req
                .headers()
                .get("operation_id")
                .and_then(|v| v.to_str().ok());

            let _op_id = match op_id_header {
                Some(id) => id.to_string(),
                None => {
                    let status =
                        tonic::Status::permission_denied(r#"No "operation_id" was provided"#);
                    return Ok(status.into_http()); // Convertimos Status a Response HTTP
                }
            };
            let _jwt_claims = match req.extensions().get::<JWTClaims>() {
                Some(jwt_claims) => jwt_claims,
                None => {
                    let status = tonic::Status::internal(r#"Missing a required request extension "jwt_claims", authentication middleware might not be working"#);
                    return Ok(status.into_http());
                },
            };

            // ! ⚠️ IMPLEMENTAR LA LLAMADA A METADATA SERVER PARA VERIFICAR PERMISO DE OPERACIÓN, EXISTENCIA Y COINCIDENCIA
            let allowed = true; // Simulación del resultado del otro servidor

            if !allowed {
                let status = tonic::Status::permission_denied("Operación no autorizada");
                return Ok(status.into_http());
            }
    
            let response: http::Response<ResBody> = inner.call(req).await?;
            Ok(response)
        })
    }
}
