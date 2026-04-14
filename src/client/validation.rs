use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Type)]
enum Role {
    User,
    Admin,
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    username_hash: String,
    password_hash: String,
    user_role: Role,
}

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
/*
Verificar que es articulo scientifico si o si, con:

antecedentes
marco teorico
antecedentes
resultados
conclusiones
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct SubTopic(String);
