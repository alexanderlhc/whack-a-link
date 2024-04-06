use reqwest::StatusCode;

use crate::common::helpers::run_server;

mod common;

#[tokio::test]
async fn test_health_check_exist() {
    let test_server = run_server().await;

    let res = test_server.get("/health").await;
    let status = res.status();
    let body = res.text().await.unwrap();

    assert_eq!(body, "OK");
    assert_eq!(status, StatusCode::OK);
}
