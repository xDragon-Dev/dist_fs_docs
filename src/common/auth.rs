use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use jsonwebtoken::{decode, encode};

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use std::env;

pub fn verificate_jwt<T: serde::de::DeserializeOwned>(
    jwt: impl AsRef<[u8]>,
) -> Result<T, jsonwebtoken::errors::Error> {
    let secret = env::var("SECRET").expect("Enviroment variable \"SECRET\" must be set");
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    let jwt_claims = decode::<T>(jwt, &decoding_key, &validation)?;
    Ok(jwt_claims.claims)
}

pub fn generate_jwt(
    jwt_claims: impl serde::Serialize,
) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("SECRET").expect("Enviroment variable \"SECRET\" must be set");
    let jwt = encode(
        &Header::default(),
        &jwt_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;
    Ok(jwt)
}

pub fn hash_password(password: &[u8]) -> Result<String, argon2::password_hash::Error> {
    let argon2 = argon2::Argon2::default();
    let salt_str = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(password, &salt_str)?.to_string();
    Ok(password_hash)
}

pub fn verify_password_hash(
    password: &[u8],
    password_hash: &str,
) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    Argon2::default().verify_password(password, &parsed_hash)?;
    Ok(())
}
