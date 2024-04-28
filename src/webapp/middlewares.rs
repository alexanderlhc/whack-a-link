use axum::{extract::Request, middleware::Next};
use tracing::info;

pub async fn access_log(request: Request, next: Next) -> axum::http::Response<axum::body::Body> {
    let uri = request.uri().path().to_string();
    let start = std::time::Instant::now();
    let response = next.run(request).await;
    let elapsed = start.elapsed();

    info!(
        "{} {} {}ms",
        response.status().as_u16(),
        uri,
        elapsed.as_millis()
    );

    response
}
