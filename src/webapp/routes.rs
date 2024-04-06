use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::domain::{shortcode::ShortCode, shorturl::ShortUrl, url::Url};

use super::webapp::AppState;

async fn shorten(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateShortUrl>,
) -> (StatusCode, String) {
    let data = ShortCode(body.url);
    let url = Url::parse(&format!("{}:{}", state.base_url, state.port));
    let short_url = ShortUrl::new(url.unwrap(), data);

    (StatusCode::CREATED, short_url.to_url())
}

#[derive(Deserialize)]
struct CreateShortUrl {
    url: String,
}

async fn read_shortcode(
    Path(shortcode): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Redirect {
    let short_url = ShortUrl::from_shortcode(&state.base_url, &shortcode);
    let url = format!("{}:{}/health", &state.base_url, &state.port);
    Redirect::permanent(&url)
}

async fn health_check() -> &'static str {
    "OK"
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/shorten", post(shorten))
        .route("/:shortcode", get(read_shortcode))
        .with_state(app_state)
}
