#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Failed to authenticate `{0}`")]
    Unauthorized(String),

    #[error("Not found resource `{0}`")]
    ResourceNotFound(String),

    #[error("The external layer `{0}` has failed with `{0}`")]
    External(&'static str, String),

    #[error("Failed to deserialize data from `{0}`")]
    Deserialization(#[from] serde_json::Error),

    #[error("Unexpected error `{0}`")]
    Unexpected(String),

    #[error("The value `{0}` is not valid for `{1}`")]
    InvalidValue(&'static str, String),
}

pub type Result<T> = core::result::Result<T, AppError>;
