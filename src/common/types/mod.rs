pub mod jwt_types;
pub mod sql_types;

/*
use chrono::prelude::*;
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    user_name: String,
    password_hash: String,
    user_role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DocumentType {
    OriginalArticle,
    Review,
    CaseReport,
    Letter,
    Editorial,
    ConferencePaper,
    Thesis,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct SubTopic(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct ScientificDocument {
    pub id: Uuid,
    pub posted_by: String,
    pub title: String,
    pub authors: Vec<String>,
    pub r#abstract: String,
    pub keywords: Vec<String>,
    pub topics: Vec<Topic>,
    pub sub_topics: Vec<SubTopic>,
    pub document_type: DocumentType,
    pub publication_date: chrono::DateTime<Utc>,
    pub language: String,
}
*/
