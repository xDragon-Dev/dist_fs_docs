use regex::Regex;
use std::sync::LazyLock;

pub static USER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._]{3,16}$").unwrap());

pub fn password_check(string: &str) -> Result<(), validator::ValidationError> {
    let has_numeric = string.chars().any(|c| c.is_numeric());
    let has_upper = string.chars().any(|c| c.is_uppercase());
    let has_lower = string.chars().any(|c| c.is_lowercase());
    let has_spec = string.chars().any(|c| "!@#$%^&*_-+=".contains(c));

    if !has_numeric || !has_upper || !has_lower || !has_spec {
        return Err(validator::ValidationError::new("password_complexity")
        .with_message("Password must have at least one lower case, one upper case, one number and one special character (!@#$%^&*_-+=)".into()));
    }
    if string
        .chars()
        .any(|c| !c.is_alphanumeric() && !"!@#$%^&*_-+=".contains(c))
    {
        return Err(validator::ValidationError::new("invalid_chars")
            .with_message("Password has invalid characters".into()));
    }
    Ok(())
}
