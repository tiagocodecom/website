use crate::shared::api::models::Error;
use crate::shared::api::utils::HttpClient;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait(?Send)]
pub trait ApiGetService<T> {
    fn new(http_client: Arc<HttpClient>) -> Self;

    async fn fetch(&self, endpoint: &str) -> Result<T, Error>;
}
