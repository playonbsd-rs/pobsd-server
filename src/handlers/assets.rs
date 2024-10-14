use axum::{
    http::{header, HeaderMap},
    response::Response,
};

pub async fn get_bulma_css() -> axum::http::Response<String> {
    let body = include_str!("static/bulma.min.css").to_string();
    Response::builder()
        .header("Content-Type", "text/css")
        .body(body)
        .unwrap()
}

pub async fn get_awesome_css() -> axum::http::Response<String> {
    let body = include_str!("static/fontawesome5.min.css").to_string();
    Response::builder()
        .header("Content-Type", "text/css")
        .body(body)
        .unwrap()
}

pub async fn get_charts_css() -> axum::http::Response<String> {
    let body = include_str!("static/charts.min.css").to_string();
    Response::builder()
        .header("Content-Type", "text/css")
        .body(body)
        .unwrap()
}

pub async fn get_fa_solid_900() -> (HeaderMap, Vec<u8>) {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "font/woff2".parse().unwrap());
    let body = include_bytes!("static/fa-solid-900.woff2").to_vec();
    (headers, body)
}

pub async fn get_favicon() -> (HeaderMap, Vec<u8>) {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "image/png".parse().unwrap());
    let body = include_bytes!("static/favicon.ico").to_vec();
    (headers, body)
}
