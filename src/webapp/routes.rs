use axum::{http::StatusCode, Json};
use serde::Deserialize;

use crate::domain::{shortcode::ShortCode, shorturl::ShortUrl, url::Url};

pub async fn shorten(Json(body): Json<CreateShortUrl>) -> (StatusCode, String) {
    let data = ShortCode(body.url);
    let url = Url::parse("https://www.rust-lang.org");
    let short_url = ShortUrl::new(url.unwrap(), data);

    (StatusCode::CREATED, short_url.to_url())
}

#[derive(Deserialize)]
pub struct CreateShortUrl {
    url: String,
}
