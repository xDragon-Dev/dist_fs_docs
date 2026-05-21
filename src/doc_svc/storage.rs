mod instructions;
mod private;

pub mod server {
    pub use super::Storage;
    pub use crate::common::proto::storage_instructions::server::StorageInstructionsServer;
    pub use crate::common::proto::storage_private::server::StoragePrivateServer;
}

pub mod client {
    pub use crate::common::proto::storage_private::client::*;
}
pub struct Storage;
