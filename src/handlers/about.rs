use askama_axum::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {}

pub fn about_page() -> Html<String> {
    let about = AboutTemplate {};
    Html(about.render().unwrap())
}
