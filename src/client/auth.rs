use std::env;
use std::str::FromStr;

use chrono::{Duration, Utc};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use jsonwebtoken::{decode, encode};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JWTClaims {
    pub sub: Uuid,
    pub user_role: Role,
    pub exp: i64,
}

pub fn _generate_jwt(jwt_claims: JWTClaims) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("SECRET").expect("Enviroment variable \"SECRET\" must be set");
    let jwt = encode(
        &Header::default(),
        &jwt_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;
    Ok(jwt)
}

pub fn _verificate_jwt(jwt: String) -> Result<JWTClaims, jsonwebtoken::errors::Error> {
    let secret = env::var("SECRET").expect("Enviroment variable \"SECRET\" must be set");
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

    let jwt_claims = decode::<JWTClaims>(&jwt, &decoding_key, &validation)?;
    Ok(jwt_claims.claims)
}
