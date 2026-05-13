mod private;
mod public;

pub mod prelude {
    pub use super::Metadata;
    pub use super::private::MetadataPrivateServer;
    pub use super::public::MetadataPublicServer;
}

pub struct Metadata {
    pub pg_pool: sqlx::Pool<sqlx::Postgres>,
}
