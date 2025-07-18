#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::middleware::Compress;
    use actix_web::*;
    use dotenvy::dotenv;
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_meta::MetaTags;
    use std::env;

    use website::adapters::driven::drupal_jsonapi::services::{Config, HttpClientService};
    use website::adapters::driver::leptos_webui::views::app::*;

    dotenv().ok();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    HttpServer::new(move || {
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();
        let api_base_url = env::var("JSONAPI_BASE_URL").expect("JSONAPI_BASE_URL is undefined");
        let api_username = env::var("JSONAPI_USERNAME").expect("JSONAPI_USERNAME is undefined");
        let api_password =
            env::var("JSONAPI_PASSWORD").expect("JSONAPI_PASSWORD is undefined");

        let client_config = Config::default()
            .base_url(api_base_url.clone().into())
            .basic_auth((api_username.as_str(), api_password.as_str()))
            .build();
        let http_client = HttpClientService::new(client_config);

        println!("listening on http://{}", &addr);

        App::new()
            .wrap(Compress::default())
            .service(favicon)
            .service(Files::new("/assets", &site_root))
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8"/>
                                <link rel="preconnect" href="https://fonts.googleapis.com" />
                                <link rel="preconnect" href=api_base_url.clone() />
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone()/>
                                <MetaTags/>
                            </head>
                            <body>
                                <App/>
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(http_client.to_owned()))
            .app_data(web::Data::new(leptos_options.to_owned()))
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use website::adapters::driver::leptos_webui::views::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
