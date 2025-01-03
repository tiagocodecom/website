use lazy_static::lazy_static;

lazy_static! {}

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::middleware::Compress;
    use actix_web::*;
    use leptos::prelude::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use tiagocode_website::app::*;
    use tiagocode_website::shared::api::utils::{ClientConfigBuilder, HttpClient};

    let leptos_config = get_configuration(None).unwrap();
    let site_address = leptos_config.leptos_options.site_addr;
    let leptos_routes = generate_route_list(App);

    println!("listening on http://{}", &site_address);

    let client_config = ClientConfigBuilder::default()
        .base_url(Some("https://local-admin.tiagocode.com".to_string()))
        .build()
        .unwrap();
    let http_client = HttpClient::new(client_config).unwrap();

    HttpServer::new(move || {
        let leptos_options = &leptos_config.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .wrap(Compress::default())
            .service(favicon)
            .service(Files::new("/assets", site_root.as_ref()))
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .leptos_routes(leptos_routes.to_owned(), {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
            .app_data(web::Data::new(http_client.clone()))
    })
    .bind(&site_address)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::prelude::LeptosOptions>,
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
    use temp::app::*;

    console_error_panic_hook::set_once();

    leptos::hydrate_body(App);
}
