fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = [
        "proto/metadata.proto",
        "proto/storage.proto",
        "proto/metadata_replication.proto",
        "proto/storage_replication.proto",
        "proto/storage_metadata.proto",
    ];

    tonic_prost_build::configure()
        .compile_protos(&protos, &["proto"])
        .unwrap();
    Ok(())
}
