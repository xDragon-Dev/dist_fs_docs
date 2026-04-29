use regex::Regex;
use std::sync::LazyLock;

/*
//Requiere al menos 8 caracteres, una letra mayúscula, una minúscula, un número y al menos un carácter especial (@$!%*?&)
pub static PASSWORD_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$").unwrap()
});
*/

pub static USER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._]{3,16}$").unwrap());

pub fn has_lowercase(string: &str) -> Result<(), validator::ValidationError> {
    for c in string.chars() {
        if c.is_lowercase() {
            return Ok(());
        }
    }
    let error = validator::ValidationError::new("no lowercase")
        .with_message("Field must contain at least one lowercase character".into());
    Err(error)
}

pub fn has_uppercase(string: &str) -> Result<(), validator::ValidationError> {
    for c in string.chars() {
        if c.is_uppercase() {
            return Ok(());
        }
    }
    let error = validator::ValidationError::new("no uppercase")
        .with_message("Field must contain at least one uppercase character".into());
    Err(error)
}

pub fn has_numeric(string: &str) -> Result<(), validator::ValidationError> {
    for c in string.chars() {
        if c.is_numeric() {
            return Ok(());
        }
    }
    let error = validator::ValidationError::new("no numeric")
        .with_message("Field must contain at least one numeric character".into());
    Err(error)
}

pub fn has_special(string: &str) -> Result<(), validator::ValidationError> {
    for c in string.chars() {
        if "!@#$%^&*_-+=".contains(c) {
            return Ok(());
        }
    }
    let error = validator::ValidationError::new("no special")
        .with_message("Field must contain at least one special character (!@#$%^&*_-+=)".into());
    Err(error)
}
