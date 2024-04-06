use reqwest::StatusCode;

use crate::common::helpers::run_server;

mod common;

#[tokio::test]
async fn test_health_check_exist() {
    let test_server = run_server().await;
    let url = format!("{}/health", test_server.address);

    let req = reqwest::get(url).await.unwrap();
    let status = req.status();
    let body = req.text().await.unwrap();

    assert_eq!(body, "OK");
    assert_eq!(status, StatusCode::OK);
}
