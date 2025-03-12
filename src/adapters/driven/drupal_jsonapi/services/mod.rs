pub mod http_client_service;
mod jsonapi_client_service;

pub use http_client_service::{BasicAuth, Config, HttpClientService};
pub use jsonapi_client_service::*;
