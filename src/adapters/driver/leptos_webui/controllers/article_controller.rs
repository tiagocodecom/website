use leptos::prelude::ServerFnError;
use leptos::prelude::*;

use crate::application::domain::article::{Article, Category};
use crate::application::domain::page::Page;

#[server]
pub async fn articles_list_controller(
    slug: String,
) -> Result<(Page, Vec<Category>, Vec<Article>), ServerFnError> {
    use actix_web::web::Data;
    use leptos::logging::error;
    use leptos_actix::extract;

    use crate::adapters::driven::drupal_jsonapi::repositories::ArticleRepository;
    use crate::adapters::driven::drupal_jsonapi::repositories::CategoryRepository;
    use crate::adapters::driven::drupal_jsonapi::repositories::PageRepository;
    use crate::application::domain::core::AppError;
    use crate::application::ports::driver::ForDisplayingArticlesList;
    use crate::application::use_cases::ShowArticlesListUseCase;
    use crate::utilities::HttpClient;

    let http_client: Data<HttpClient> = extract().await?;

    let page_repository = PageRepository::new(http_client.get_ref().clone());
    let article_repository = ArticleRepository::new(http_client.get_ref().clone());
    let category_repository = CategoryRepository::new(http_client.get_ref().clone());

    let use_case = ShowArticlesListUseCase::new(
        Box::new(article_repository),
        Box::new(category_repository),
        Box::new(page_repository),
    );

    let category = slug.clone().split("/").nth(3).map(|s| s.to_owned());
    let result = use_case
        .execute("/en/articles", category)
        .await
        .map_err(|e| {
            error!("{}", e.to_string());
            ServerFnError::<AppError>::ServerError(e.to_string())
        })?;

    Ok(result)
}

#[server]
pub async fn article_detail_controller(slug: String) -> Result<Article, ServerFnError> {
    use actix_web::web::Data;
    use leptos::logging::error;
    use leptos_actix::extract;

    use crate::adapters::driven::drupal_jsonapi::repositories::ArticleRepository;
    use crate::application::domain::core::AppError;
    use crate::application::ports::driver::ForDisplayingArticle;
    use crate::application::use_cases::ShowArticleDetailUseCase;
    use crate::utilities::HttpClient;

    let http_client: Data<HttpClient> = extract().await?;
    let article_repository = ArticleRepository::new(http_client.get_ref().clone());

    let use_case = ShowArticleDetailUseCase::new(Box::new(article_repository));

    let result = use_case.execute(slug.as_str()).await.map_err(|e| {
        error!("{}", e.to_string());
        ServerFnError::<AppError>::ServerError(e.to_string())
    })?;

    Ok(result)
}
