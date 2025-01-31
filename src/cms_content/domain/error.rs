#[derive(thiserror::Error, Debug)]
pub enum CmsError {
    #[error("Failed to authenticate `{0}`")]
    Unauthorized(String),
    #[error("Not found resource `{0}`")]
    ResourceNotFound(String),
    #[error("Failed communication with `{0}`")]
    CommunicationFailed(String),
    #[error("Unexpected error `{0}`")]
    Unknown(String),
}

pub type CmsResult<T> = Result<T, CmsError>;
