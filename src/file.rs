use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum::{
        body::{boxed, Body, BoxBody},
        extract::Extension,
        http::{Request, Response, StatusCode, Uri},
    };
    use tower::ServiceExt;
    use tower_http::services::ServeDir;
    use std::sync::Arc;
    use leptos::LeptosOptions;

    pub async fn file_handler(uri: Uri, Extension(options): Extension<Arc<LeptosOptions>>) -> Result<Response<BoxBody>, (StatusCode, String)> {
        let options = &*options;
        let root = options.site_root.clone();
        let res = get_static_file(uri.clone(), &root).await?;

        match res.status() {
            StatusCode::OK => Ok(res),
            _ => Err((res.status(), "File Not Found".to_string())),
        }
    }

    pub async fn get_static_file(uri: Uri, root: &str) -> Result<Response<BoxBody>, (StatusCode, String)> {
        let req = Request::builder()
            .uri(uri.clone())
            .body(Body::empty())
            .unwrap();
        let root_path = format!("{root}");
        match ServeDir::new(&root_path).oneshot(req).await {
            Ok(res) => Ok(res.map(boxed)),
            Err(err) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", err),
            ))
        }
    }

}
}
