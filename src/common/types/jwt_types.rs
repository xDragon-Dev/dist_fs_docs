use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Role {
    User = 0,
    Admin = 1,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub role: Role,
    pub exp: i64,
    pub iat: i64,
}

impl core::convert::TryFrom<i32> for Role {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Role::User),
            1 => Ok(Role::Admin),
            _ => Err("Bad enum conversion"),
        }
    }
}
