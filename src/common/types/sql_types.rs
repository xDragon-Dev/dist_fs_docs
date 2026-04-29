use chrono::prelude::*;
use sqlx::prelude::{FromRow, Type};
use sqlx::types::Json;

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
