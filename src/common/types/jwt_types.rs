use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Deserialize, Serialize, Type, Clone, PartialEq)]
#[sqlx(type_name = "role")]
pub enum TokenRole {
    User = 0,
    Admin = 1,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TokenClaims {
    pub sub: String,
    pub user_role: TokenRole,
    pub exp: i64,
}

impl core::convert::TryFrom<i32> for TokenRole {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TokenRole::User),
            1 => Ok(TokenRole::Admin),
            _ => Err("Bad enum conversion"),
        }
    }
}
