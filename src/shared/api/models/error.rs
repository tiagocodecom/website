#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("`{0}")]
    NotFound(String),

    #[error("communication failed with code `{0}`, and message `{1}`")]
    RequestFailed(u16, String),

    #[error("deserialization failed for the resource `{0}`")]
    DeserializationFailed(String),

    #[error("url is invalid `{0}`")]
    InvalidUrl(String),

    #[error("response data isn't in a valid format `{0}`")]
    InvalidResponse(String),

    #[error("request data is invalid `{0}`")]
    InvalidRequest(String),

    #[error("unknown error `{0}`")]
    Unknown(String),
}
