mod private;
mod public;
mod middleware;

pub mod server {
    pub use super::Metadata;
    pub use crate::common::proto::metadata_private::server::MetadataPrivateServer;
    pub use crate::common::proto::metadata_public::server::MetadataPublicServer;

    pub use super::middleware::AuthLayer;
}

pub mod client {
    pub use crate::common::proto::metadata_private::client::*;
    pub use crate::common::proto::metadata_public::client::*;
}

pub struct Metadata {
    pub pg_pool: sqlx::Pool<sqlx::Postgres>,
}

impl Metadata {
    pub fn new(pg_pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pg_pool }
    }
}
