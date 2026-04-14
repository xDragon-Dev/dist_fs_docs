fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = [
        "proto/client_metadata.proto",
        "proto/client_storage.proto",
        "proto/metadata_metadata.proto",
        "proto/storage_storage.proto",
    ];

    tonic_prost_build::configure()
        .compile_protos(&protos, &["proto"])
        .unwrap();
    Ok(())
}
