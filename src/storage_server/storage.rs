mod instructions;
mod private;

pub mod prelude {
    pub use super::Storage;
    pub use super::instructions::StorageInstructionsServer;
    pub use super::private::StoragePrivateServer;
}
pub struct Storage;
