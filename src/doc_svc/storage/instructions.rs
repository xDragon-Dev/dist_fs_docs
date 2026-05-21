use crate::common::proto::storage_instructions::service::*;

use std::path::Path;
use sysinfo::{CpuRefreshKind, DiskRefreshKind, Disks, RefreshKind, System};
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl StorageInstructions for super::Storage {
    async fn delete_files(
        &self,
        request: Request<DeleteFilesRequest>,
    ) -> Result<Response<()>, Status> {
        let request = request.into_inner();
        let mut messages = Vec::new();
        for file in request.file_ids {
            if let Err(_) = tokio::fs::remove_file(&file).await {
                let err_message = format!("Failed deleting file {}", file);
                messages.push(err_message);
                continue;
            }
        }
        if !messages.is_empty() {
            return Err(Status::data_loss(messages.join("\n")));
        }
        Ok(Response::new(()))
    }

    async fn heart_beat(&self, _request: Request<()>) -> Result<Response<StorageStatus>, Status> {
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::nothing().with_cpu_usage()),
        );
        tokio::time::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;
        sys.refresh_cpu_usage();
        let cpu_usage = sys.global_cpu_usage();

        let disks =
            Disks::new_with_refreshed_list_specifics(DiskRefreshKind::nothing().with_storage());

        let available_space = disks
            .iter()
            .find(|d| d.mount_point() == Path::new("/"))
            .unwrap_or(disks.list().first().unwrap())
            .available_space();

        let response = StorageStatus {
            cpu_usage: cpu_usage as u32,
            available_space,
        };
        Ok(Response::new(response))
    }
}
