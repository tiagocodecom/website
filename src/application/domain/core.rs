#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Failed to authenticate `{0}`")]
    Unauthorized(String),

    #[error("Not found resource `{0}`")]
    ResourceNotFound(String),

    #[error("The external layer `{0}` has failed with `{1}`")]
    External(&'static str, String),

    #[error("Deserialization error `{0}` with data `{1}`")]
    Deserialization(String, String),

    #[error("Unexpected error `{0}`")]
    Unexpected(String),

    #[error("The value `{0}` is not valid for `{1}`")]
    InvalidValue(&'static str, String),
}

pub type Result<T> = core::result::Result<T, AppError>;
