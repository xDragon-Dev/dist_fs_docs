mod private;
mod public;

pub mod prelude {
    pub use super::Metadata;
    pub use super::private::PrivateMetadataServer;
    pub use super::public::PublicMetadataServer;
}

pub struct Metadata {
    pub pg_pool: sqlx::Pool<sqlx::Postgres>,
}
