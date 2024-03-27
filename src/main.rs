use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use whack_a_link::domain::{shortcode::ShortCode, shorturl::ShortUrl, url::Url};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/api/shorten", post(shorten));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn shorten(Json(body): Json<CreateShortUrl>) -> (StatusCode, String) {
    let data = ShortCode(body.url);
    let url = Url::parse("https://www.rust-lang.org");
    let short_url = ShortUrl::new(url.unwrap(), data);

    println!("Shortened URL: {}", short_url.to_url());

    (StatusCode::CREATED, short_url.to_url())
}

#[derive(Deserialize)]
struct CreateShortUrl {
    url: String,
}
