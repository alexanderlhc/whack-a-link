use reqwest::StatusCode;
use serde_json::json;

use crate::common::helpers::run_server;

mod common;

#[tokio::test]
async fn test_can_create_shortened_url() {
    let server = run_server().await;

    let res = server
        .post("/shorten", json!({ "url": "https://www.rust-lang.org" }))
        .await;

    let status = res.status();
    let shortened_url = res.text().await.unwrap();

    assert_eq!(shortened_url, format!("{}/9232f01a", server.address));
    assert_eq!(status, StatusCode::CREATED);

    let visit_shortened_url = reqwest::get(&shortened_url).await.unwrap();

    assert_eq!(
        visit_shortened_url.url().as_str(),
        "https://www.rust-lang.org/"
    );
    assert_eq!(visit_shortened_url.status(), StatusCode::PERMANENT_REDIRECT);
}
