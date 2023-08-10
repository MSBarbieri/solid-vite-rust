use anyhow::Result;
use axum::{
    extract::Path,
    http::Response,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

async fn index() -> impl IntoResponse {
    let html = tokio::fs::read_to_string("web/dist/index.html")
        .await
        .unwrap();
    Html(html)
}

pub fn find_content_type<'a>(path: String) -> Result<&'a str> {
    Ok(match path.split('.').last() {
        Some(t) => match t {
            "css" => "text/css;charset=utf-8",
            "js" => "application/javascript;charset=utf-8",
            "svg" => "image/svg+xml",
            e => panic!("invalid format! {e}"),
        },
        None => panic!("invalid file"),
    })
}
async fn public_files(Path(files): Path<String>) -> impl IntoResponse {
    let path = format!("web/dist/{}", files);
    let file = tokio::fs::read_to_string(path).await.unwrap();

    Response::builder()
        .header("content-type", find_content_type(files).unwrap())
        .body(file)
        .unwrap()
}

async fn assets(Path(path): Path<String>) -> impl IntoResponse {
    let r_path = format!("web/dist/assets/{}", path);
    let file = tokio::fs::read_to_string(&r_path).await.unwrap();

    Response::builder()
        .header("content-type", find_content_type(r_path).unwrap())
        .body(file)
        .unwrap()
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(index))
        .route("/*files", get(public_files))
        .route("/assets/*path", get(assets));

    let addr = "0.0.0.0:3000".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
