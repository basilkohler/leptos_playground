use cfg_if::cfg_if;
use leptos::*;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum::{
        routing::post,
        extract::Extension,
        Router
    };
    use leptos_playground::app::{App, AppProps};
    use leptos_playground::file::file_handler;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use std::sync::Arc;

    #[tokio::main]
    async fn main() {
        simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");

        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let addr = leptos_options.site_address.clone();
        let routes = generate_route_list(|cx| view! {cx, <App/> }).await;

        let app = Router::new()
                    // .route("/api/*fn_name")
                    .leptos_routes(leptos_options.clone(), routes, |cx| view! {cx, <App/> })
                    .fallback(file_handler)
                    .layer(Extension(Arc::new(leptos_options)));

        log!("Listening on {}", &addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
} else {
    use leptos_playground::app::{App, AppProps};
    use leptos::*;
    pub fn main() {
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();
        mount_to_body(|cx| view! { cx, <App/> })
    }
}}
