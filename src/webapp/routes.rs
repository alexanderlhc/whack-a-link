use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::domain::{shortcode::ShortCode, shorturl::ShortUrl, url::Url};

async fn shorten(Json(body): Json<CreateShortUrl>) -> (StatusCode, String) {
    let data = ShortCode(body.url);
    let url = Url::parse("https://www.rust-lang.org");
    let short_url = ShortUrl::new(url.unwrap(), data);

    (StatusCode::CREATED, short_url.to_url())
}

#[derive(Deserialize)]
struct CreateShortUrl {
    url: String,
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/api/shorten", post(shorten))
}
