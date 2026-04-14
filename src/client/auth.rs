use std::str::FromStr;

use chrono::{Duration, Utc};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use jsonwebtoken::{decode, encode};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

const SECRET: &[u8; 29] = b"Mi clave super dooper secreta";

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

impl std::default::Default for JWTClaims {
    //This is meant for testing porpuses only
    fn default() -> Self {
        let exp = Utc::now()
            .checked_add_signed(Duration::minutes(5))
            // We expect this will never happen
            .expect("Date out of the 64 bit bounds")
            .timestamp();

        Self {
            sub: Uuid::from_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap(),
            user_role: Role::User,
            exp,
        }
    }
}

pub fn generate_jwt(jwt_claims: JWTClaims) -> Result<String, jsonwebtoken::errors::Error> {
    let jwt = encode(
        &Header::default(),
        &jwt_claims,
        &EncodingKey::from_secret(SECRET),
    )?;
    Ok(jwt)
}

pub fn _verificate_jwt(jwt: String) -> Result<JWTClaims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(SECRET);
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

    let jwt_claims = decode::<JWTClaims>(&jwt, &decoding_key, &validation)?;
    Ok(jwt_claims.claims)
}
