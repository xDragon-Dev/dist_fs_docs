pub mod metadata_private {
    mod metadata_private_proto {
        tonic::include_proto!("metadata_private");
    }

    pub mod service {
        pub use super::metadata_private_proto::metadata_private_server::MetadataPrivate;

        pub use super::metadata_private_proto::{
            AssignedNode, ChangePasswordRequest, ChangeUserNameRequest, CreateSubTopicRequest,
            CreateTopicRequest, DeleteDocumentRequest, DeleteSubTopicRequest, DeleteTopicRequest,
            DeleteUserRequest, DownloadNodeRequest, ScientificDocument, ScientificDocumentRequest,
            SearchRequest, SearchResult, SearchResults, SubTopic, SubTopics, Topic, Topics,
            UploadNodeRequest,
        };
    }

    pub mod client {
        pub use super::metadata_private_proto::metadata_private_client::MetadataPrivateClient;

        pub use super::metadata_private_proto::{
            ChangePasswordRequest, ChangeUserNameRequest, CreateSubTopicRequest,
            CreateTopicRequest, DeleteDocumentRequest, DeleteSubTopicRequest, DeleteTopicRequest,
            DeleteUserRequest, DownloadNodeRequest, ScientificDocumentRequest, SearchRequest,
            UploadNodeRequest,
        };
    }

    pub mod server {
        pub use super::metadata_private_proto::metadata_private_server::MetadataPrivateServer;
    }
}

pub mod storage_private {
    mod storage_private_proto {
        tonic::include_proto!("storage_private");
    }

    pub mod service {
        pub use super::storage_private_proto::storage_private_server::StoragePrivate;

        pub use super::storage_private_proto::upload_chunk::Data;
        pub use super::storage_private_proto::{
            DownloadChunk, DownloadFileRequest, UploadChunk, //UploadFooter
        };
    }

    pub mod client {
        pub use super::storage_private_proto::storage_private_client::StoragePrivateClient;

        pub use super::storage_private_proto::{
            DownloadFileRequest, UploadChunk, UploadFooter, upload_chunk::Data,
        };
    }

    pub mod server {
        pub use super::storage_private_proto::storage_private_server::StoragePrivateServer;
    }
}

pub mod storage_instructions {
    mod storage_instructions_proto {
        tonic::include_proto!("storage_instructions");
    }
    pub mod service {
        pub use super::storage_instructions_proto::storage_instructions_server::StorageInstructions;
        pub use super::storage_instructions_proto::{DeleteFilesRequest, StorageStatus};
    }

    pub mod client {
        pub use super::storage_instructions_proto::DeleteFilesRequest;
        pub use super::storage_instructions_proto::storage_instructions_client::StorageInstructionsClient;
    }

    pub mod server {
        pub use super::storage_instructions_proto::storage_instructions_server::StorageInstructionsServer;
    }
}

pub mod metadata_public {
    mod metadata_public_proto {
        tonic::include_proto!("metadata_public");
    }

    pub mod service {
        pub use super::metadata_public_proto::metadata_public_server::MetadataPublic;

        pub use super::metadata_public_proto::{
            CreateUserRequest, LogInRequest, SubTopic, SubTopics, Topic, Topics,
        };
    }

    pub mod client {
        pub use super::metadata_public_proto::metadata_public_client::MetadataPublicClient;
        pub use super::metadata_public_proto::{CreateUserRequest, LogInRequest};
    }

    pub mod server {
        pub use super::metadata_public_proto::metadata_public_server::MetadataPublicServer;
    }
}
