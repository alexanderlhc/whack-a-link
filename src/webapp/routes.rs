use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use tracing::info;

use crate::{
    domain::{
        shortcode::{Hash, ShortCode},
        shorturl::ShortUrl,
        url::Url,
    },
    storage::storage_shortcode::{get_url_by_shortcode, insert_url},
};

use super::{middlewares::access_log, webapp::AppState};

async fn shorten(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateShortUrl>,
) -> (StatusCode, String) {
    let destination = body.url.clone();
    let data = ShortCode(destination.clone());
    let url = Url::parse(&format!("{}:{}", state.base_url, state.port)).unwrap();
    let short_url = ShortUrl::new(url.clone(), data);
    let data = ShortCode(destination.clone());
    insert_url(&data.compress(), &destination, &state.db)
        .await
        .unwrap();

    info!("shortened {} to: {}", destination, short_url.to_url());

    (StatusCode::CREATED, short_url.to_url())
}

#[derive(Deserialize)]
struct CreateShortUrl {
    url: String,
}

struct NotFoundError;
impl IntoResponse for NotFoundError {
    fn into_response(self) -> Response {
        (StatusCode::NOT_FOUND, "Not Found").into_response()
    }
}

async fn read_shortcode(
    Path(shortcode): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Redirect, NotFoundError> {
    let url = get_url_by_shortcode(&shortcode, &state.db).await.unwrap();
    match url {
        Some(url) => Ok(Redirect::permanent(&url.url)),
        None => Err(NotFoundError),
    }
}

async fn health_check() -> &'static str {
    "OK"
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/shorten", post(shorten))
        .route("/:shortcode", get(read_shortcode))
        .layer(middleware::from_fn(access_log))
        .with_state(app_state)
}
