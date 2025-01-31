use crate::shared::api::models::{Error, MenuItems};
use crate::shared::api::services::ApiGetService;
use crate::shared::api::utils::HttpClient;
use async_trait::async_trait;
use std::sync::Arc;

pub struct MenuItemsService {
    http_client: Arc<HttpClient>,
}

#[async_trait(?Send)]
impl ApiGetService<MenuItems> for MenuItemsService {
    fn new(http_client: Arc<HttpClient>) -> Self {
        Self { http_client }
    }

    async fn fetch(&self, endpoint: &str) -> Result<MenuItems, Error> {
        let response = self
            .http_client
            .get(&format!("/api/menu_items/{endpoint}"))
            .await?;

        if response.status().as_u16() == 404 {
            return Err(Error::NotFound(format!("Menu {endpoint} not found")));
        }

        let response = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| Error::InvalidResponse(e.to_string()))?;

        Ok(serde_json::from_value::<MenuItems>(response)
            .map_err(|e| Error::DeserializationFailed(e.to_string()))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::api::utils::{ClientConfigBuilder, HttpClient};
    use mockito::{Mock, Server};
    use std::sync::Arc;

    #[actix_rt::test]
    async fn gets_menu_items_successfully() {
        let mut server = Server::new_async().await;
        let http_client = setup_http_client_mock(&server.url());
        let rest_api_mock = setup_rest_api_mock(&mut server, "social-networks", 200).await;

        let menu_items = MenuItemsService::new(http_client)
            .fetch("social-networks")
            .await
            .unwrap();

        assert!(rest_api_mock.matched_async().await); // Ensures that api is called to get the data
        assert_eq!(menu_items.len(), 2); // Ensures that the response mock contains 2 elements
    }

    #[actix_rt::test]
    async fn returns_error_when_menu_is_not_found_by_restapier() {
        let mut server = Server::new_async().await;
        let http_client = setup_http_client_mock(&server.url());
        let rest_api_mock = setup_rest_api_mock(&mut server, "non-existing", 404).await;

        let result = MenuItemsService::new(http_client)
            .fetch("non-existing")
            .await;

        assert!(rest_api_mock.matched_async().await); // Ensures that api is called to get the data
        assert!(result.is_err()); // Ensures that API returns an error
        assert_eq!(
            result.unwrap_err().to_string(),
            "Menu non-existing not found"
        ); // Ensures that the error message is correct
    }

    fn setup_http_client_mock(url: &str) -> Arc<HttpClient> {
        let client_config = ClientConfigBuilder::default()
            .base_url(Some(url.to_string()))
            .build()
            .unwrap();
        Arc::new(HttpClient::new(client_config).unwrap())
    }

    async fn setup_rest_api_mock(server: &mut Server, menu_id: &str, status: usize) -> Mock {
        let endpoint = format!("/api/menu_items/{menu_id}");

        server
            .mock("GET", endpoint.as_str())
            .with_status(status)
            .with_header("Content-Type", "application/json")
            .with_body_from_file(format!("tests/fixtures/menu_items_{status}.json"))
            .create_async()
            .await
    }
}
