use serde::de::DeserializeOwned;
use serde_json::from_value;

use crate::adapters::driven::drupal_jsonapi::entities::ResolvedRoute;
use crate::utilities::HttpClient;

pub struct JsonApiClientService {
    http_client: HttpClient,
}

impl JsonApiClientService {
    pub fn new(http_client: HttpClient) -> Self {
        Self { http_client }
    }

    pub async fn resolve_external_endpoint(&self, path: &str) -> Result<String, String> {
        let json = self
            .http_client
            .get_json(&format!("/router/translate-path?path={path}"))
            .await
            .unwrap();

        let external_route =
            from_value::<ResolvedRoute>(json.clone()).map_err(|e| e.to_string())?;

        Ok(format!(
            "/{}/{}/{}/{}",
            external_route.jsonapi().path_prefix(),
            external_route.entity().entity_type(),
            external_route.entity().bundle(),
            external_route.entity().uuid()
        ))
    }

    pub async fn get_external_data<T>(&self, endpoint: &str) -> Result<T, String>
    where
        T: DeserializeOwned,
    {
        let json = self.http_client.get_json(endpoint).await.unwrap();

        serde_json_path_to_error::from_value::<T>(json.clone())
            .map_err(|e| format!(r#"{}\n{}\n{}\n"#, e.to_string(), endpoint, json))
    }
}
