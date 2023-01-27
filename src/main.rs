use cfg_if::cfg_if;
use leptos::*;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum::{
        routing::post,
        extract::Extension,
        Router,
        body::{boxed, Body, BoxBody},
        response::IntoResponse,
        http::{Request, Response, StatusCode, Uri},
    };
    use axum::response::Response as AxumResponse;
    use leptos_playground::app::{App, AppProps};
    use leptos_playground::file::file_handler;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use std::sync::Arc;
    use tower::ServiceExt;
    use tower_http::services::ServeDir;
    use leptos::{LeptosOptions};
    use leptos_playground::items::register_server_functions;


    #[tokio::main]
    async fn main() {
        simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");

        let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
        let leptos_options = conf.leptos_options;
        let addr = leptos_options.site_address.clone();
        let routes = generate_route_list(|cx| view! {cx, <App/> }).await;

        register_server_functions();

        let app = Router::new()
            .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
            .leptos_routes(leptos_options.clone(), routes, |cx| view! {cx, <App/> })
            .fallback(file_handler)
            .layer(Extension(Arc::new(leptos_options)));

        log!("Listening on {}", &addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }

    pub async fn file_and_error_handler(uri: Uri, Extension(options): Extension<Arc<LeptosOptions>>, req: Request<Body>) -> AxumResponse {
        let options = &*options;
        let root = options.site_root.clone();
        let res = get_static_file(uri.clone(), &root).await.unwrap();

        if res.status() == StatusCode::OK {
           res.into_response()
        } else{
            let handler = leptos_axum::render_app_to_stream(options.to_owned(), |cx| view! {cx, <h1>"Error"</h1>} );//error_template(cx, None));
            handler(req).await.into_response()
        }
    }

    async fn get_static_file(uri: Uri, root: &str) -> Result<Response<BoxBody>, (StatusCode, String)> {
        let req = Request::builder().uri(uri.clone()).body(Body::empty()).unwrap();
        let root_path = format!("{root}");
        // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
        // This path is relative to the cargo root
        match ServeDir::new(&root_path).oneshot(req).await {
            Ok(res) => Ok(res.map(boxed)),
            Err(err) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", err),
            )),
        }
    }
} else {
    use leptos_playground::app::{App, AppProps};
    use leptos::*;
    pub fn main() {
        console_error_panic_hook::set_once();
        _ = console_log::init_with_level(log::Level::Debug);
        mount_to_body(|cx| view! { cx, <App/> })
    }
}}
