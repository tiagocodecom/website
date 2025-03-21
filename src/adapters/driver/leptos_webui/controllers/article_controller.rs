use leptos::html::article;
use leptos::prelude::ServerFnError;
use leptos::prelude::*;

use crate::application::domain::article::{Article, Category};

#[server]
pub async fn article_controller() -> Result<(Vec<Category>, Vec<Article>), ServerFnError> {
    use actix_web::web::Data;
    use leptos::logging::error;
    use leptos_actix::extract;

    use crate::adapters::driven::drupal_jsonapi::repositories::ArticleRepository;
    use crate::adapters::driven::drupal_jsonapi::repositories::CategoryRepository;
    use crate::adapters::driven::drupal_jsonapi::services::HttpClientService;
    use crate::application::domain::core::AppError;
    use crate::application::ports::driver::ForListingArticles;
    use crate::application::use_cases::ShowArticlesUseCase;

    let http_client: Data<HttpClientService> = extract().await?;

    let article_repository = ArticleRepository::new(http_client.get_ref().clone());
    let category_repository = CategoryRepository::new(http_client.get_ref().clone());

    let use_case =
        ShowArticlesUseCase::new(Box::new(article_repository), Box::new(category_repository));

    let result = use_case.execute().await.map_err(|e| {
        error!("{}", e.to_string());
        ServerFnError::<AppError>::ServerError(e.to_string())
    })?;

    Ok(result)
}
