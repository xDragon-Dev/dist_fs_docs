use regex::Regex;
use std::sync::LazyLock;

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

pub fn has_valid_chars(string: &str) -> Result<(), validator::ValidationError> {
    let error = validator::ValidationError::new("invalid chars").with_message(
        "Field must contain alphanumeric characters and any of following: !@#$%^&*_-+=".into(),
    );
    for c in string.chars() {
        if !c.is_alphanumeric() && !"!@#$%^&*_-+=".contains(c) {
            return Err(error);
        }
    }
    Ok(())
}
