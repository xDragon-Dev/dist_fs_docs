fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = [
        "proto/metadata.proto",
        "proto/storage.proto",
        "proto/metadata_replication.proto",
        "proto/storage_replication.proto",
        "proto/storage_metadata.proto",
    ];

    tonic_prost_build::configure()
        .add_layer(sqlx_layer)
        .add_layer(validator_layer)
        .compile_protos(&protos, &["proto"])
        .unwrap();
    Ok(())
}

use tonic_prost_build::Builder;

fn sqlx_layer(builder: Builder) -> Builder {
    builder
        .message_attribute("Topic", "#[derive(sqlx::FromRow)]")
        .message_attribute("SubTopic", "#[derive(sqlx::FromRow)]")
        .enum_attribute("SearchKind", "#[derive(sqlx::Type)]")
}

fn validator_layer(builder: Builder) -> Builder {
    builder
        .message_attribute("CreateUserRequest", "#[derive(validator::Validate)]")
        .field_attribute(
            "CreateUserRequest.user_name",
            r#"#[validate(regex(path = *common::valid::USER_REGEX, message = "Invalid username"))]"#,
        )
        .field_attribute(
            "CreateUserRequest.password",
            r#"#[validate(
            length(min = 8, message = "Field must be at least 8 characters long"),
            custom(function = "common::valid::has_lowercase"),
            custom(function = "common::valid::has_uppercase"),
            custom(function = "common::valid::has_numeric"),
            custom(function = "common::valid::has_special")
        )]"#,
        )
        .field_attribute(
            "CreateUserRequest.confirm_password",
            r#"#[validate(must_match(other = "password", message = "Mismatched passwords"))]"#,
        )
}

pub trait Layer: Sized {
    fn add_layer(self, layer_fn: impl Fn(Self) -> Self) -> Self;
}

impl Layer for Builder {
    fn add_layer(self, layer_fn: impl Fn(Self) -> Self) -> Self {
        layer_fn(self)
    }
}
