mod client_storage_proto {
    tonic::include_proto!("storage");
}

use super::auth::verify_jwt;
use super::types::jwt_types::TokenClaims;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tonic::{Request, Status};
use tower::{Layer, Service};

pub fn auth_jwt(mut req: Request<()>) -> Result<Request<()>, Status> {
    let header_jwt = req
        .metadata()
        .get("jwt")
        .and_then(|result| result.to_str().ok());

    match header_jwt {
        Some(jwt) => {
            let claims = verify_jwt::<TokenClaims>(jwt)
                .map_err(|e| Status::unauthenticated(e.to_string()))?;
            req.extensions_mut().insert(claims);
        }
        None => {
            return Err(Status::unauthenticated(r#"No "jwt" was provided"#));
        }
    }

    Ok(req)
}

#[derive(Debug, Clone)]
pub struct PermisionLayer;

impl<S> Layer<S> for PermisionLayer {
    type Service = Permision<S>;

    fn layer(&self, service: S) -> Self::Service {
        Permision { inner: service }
    }
}

#[derive(Debug, Clone)]
pub struct Permision<S> {
    inner: S,
}

type BoxFuture<'a, T> = Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

impl<S, ReqBody, ResBody> Service<http::Request<ReqBody>> for Permision<S>
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
            let _jwt_claims = match req.extensions().get::<TokenClaims>() {
                Some(jwt_claims) => jwt_claims,
                None => {
                    let status = tonic::Status::internal(
                        r#"Authentication failure: Missing a required request extension "jwt_claims"#,
                    );
                    return Ok(status.into_http());
                }
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
