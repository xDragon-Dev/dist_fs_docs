mod calls;
mod checksum;

use dotenvy::dotenv;
use std::path::Path;
use sysinfo::{CpuRefreshKind, DiskRefreshKind, Disks, RefreshKind, System};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::nothing().with_cpu_usage()),
    );
    tokio::time::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;
    sys.refresh_cpu_usage();
    let cpu_usage = sys.global_cpu_usage();

    let disks = Disks::new_with_refreshed_list_specifics(DiskRefreshKind::nothing().with_storage());

    let available_space = disks
        .iter()
        .find(|d| d.mount_point() == Path::new("/"))
        .unwrap_or(disks.list().first().unwrap())
        .available_space();

    println!("cpu usage: {}%", cpu_usage);
    println!(
        "available space: {} GB",
        available_space / 1024 / 1024 / 1024
    );
    Ok(())
}
