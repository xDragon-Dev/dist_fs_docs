mod metadata_instructions;
mod storage_service;

pub mod prelude {
    pub use super::Storage;
    pub use super::metadata_instructions::MetadataInstructionsServer;
    pub use super::storage_service::StorageServer;
}
pub struct Storage;
