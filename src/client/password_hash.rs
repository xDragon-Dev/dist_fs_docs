use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub fn _hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let argon2 = argon2::Argon2::default();
    let salt_str = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt_str)?
        .to_string();
    Ok(password_hash)
}

pub fn _verify_password_hash(
    password_hash: &str,
    password: &[u8],
) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    Argon2::default().verify_password(password, &parsed_hash)?;
    Ok(())
}
