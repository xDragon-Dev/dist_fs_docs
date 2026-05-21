use chrono::prelude::*;
use sqlx::prelude::{FromRow, Type};
use sqlx::types::Json;
use uuid::Uuid;

#[derive(Debug, Type)]
#[sqlx(type_name = "document_type")]
pub enum DocumentType {
    OriginalArticle,
    Review,
    CaseReport,
    Letter,
    Editorial,
    ConferencePaper,
    Thesis,
}

#[derive(Debug, Type, PartialEq)]
#[sqlx(type_name = "role")]
pub enum Role {
    User,
    Admin,
}

#[derive(Debug, Type, PartialEq)]
#[sqlx(type_name = "scope")]
pub enum Scope {
    Local,
    Global,
}

#[derive(Debug, Type, PartialEq)]
#[sqlx(type_name = "kind")]
pub enum Kind {
    Upload,
    Download,
}

#[derive(FromRow, Debug)]
pub struct ScientificDocumentRow {
    pub posted_by: String,
    pub title: String,
    pub authors: Vec<String>,
    pub r#abstract: String,
    pub keywords: Vec<String>,
    pub topics: Json<Vec<String>>,
    pub sub_topics: Json<Vec<String>>,
    pub document_type: DocumentType,
    pub publication_date: chrono::DateTime<Utc>,
    pub language: String,
}

#[derive(FromRow, Debug)]
pub struct SearchResultRow {
    pub id: Uuid,
    pub posted_by: String,
    pub title: String,
    pub document_type: DocumentType,
    pub publication_date: chrono::DateTime<Utc>,
    pub language: String,
}

impl core::convert::Into<i32> for DocumentType {
    fn into(self) -> i32 {
        match self {
            DocumentType::OriginalArticle => 0,
            DocumentType::Review => 1,
            DocumentType::CaseReport => 2,
            DocumentType::Letter => 3,
            DocumentType::Editorial => 4,
            DocumentType::ConferencePaper => 5,
            DocumentType::Thesis => 6,
        }
    }
}

impl core::convert::TryFrom<i32> for DocumentType {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DocumentType::OriginalArticle),
            1 => Ok(DocumentType::Review),
            2 => Ok(DocumentType::CaseReport),
            3 => Ok(DocumentType::Letter),
            4 => Ok(DocumentType::Editorial),
            5 => Ok(DocumentType::ConferencePaper),
            6 => Ok(DocumentType::Thesis),
            _ => Err("Bad enum conversion"),
        }
    }
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

use super::jwt_types;

impl core::convert::Into<jwt_types::Role> for Role {
    fn into(self) -> super::jwt_types::Role {
        match self {
            Role::Admin => jwt_types::Role::Admin,
            Role::User => jwt_types::Role::User,
        }
    }
}

impl core::convert::TryFrom<i32> for Scope {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Scope::Local),
            1 => Ok(Scope::Global),
            _ => Err("Bad enum conversion"),
        }
    }
}
