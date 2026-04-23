use sha2::{Digest, Sha256};
use std::path::Path;
use tokio::{fs::File, io::AsyncReadExt};

async fn _checksum(path: impl AsRef<Path>) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(path).await?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 4096];
    while let Ok(n) = file.read(&mut buffer).await {
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    let hash = hasher.finalize();
    let hash_str = hash.iter().map(|b| format!("{:02x}", b)).collect();
    Ok(hash_str)
}
