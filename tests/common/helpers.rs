use reqwest::Response;
use sqlx::Error;
use whack_a_link::{config::AppConfig, storage::db_connect::DbCredentials, webapp::webapp::WebApp};

pub struct TestWebApp {
    pub address: String,
}

pub async fn run_server() -> TestWebApp {
    let db_credentials = create_test_db().await.unwrap();
    let port = "0".to_string(); // 0 assigns random port
    let config = AppConfig {
        port,
        db_credentials,
    };
    let webapp = WebApp::create_app(config).await.unwrap();
    let address = format!("http://localhost:{}", webapp.port);
    tokio::spawn(async move { webapp.server.await.unwrap() });

    TestWebApp { address }
}

impl TestWebApp {
    pub async fn get(&self, path: &str) -> Response {
        let url = format!("{}{}", self.address, path);
        reqwest::get(url).await.unwrap()
    }
    pub async fn post(&self, path: &str, body: serde_json::Value) -> Response {
        let url = format!("{}{}", self.address, path);
        reqwest::Client::new()
            .post(url)
            .json(&body)
            .send()
            .await
            .unwrap()
    }
}

async fn create_test_db() -> Result<DbCredentials, Error> {
    let random_db_name = format!("test_db_{}", rand::random::<u16>());
    let credentials = DbCredentials {
        database: random_db_name,
        username: "db_user".to_string(),
        password: "password".to_string(),
        host: "localhost".to_string(),
        port: "5432".to_string(),
    };

    let pool = sqlx::PgPool::connect(&format!(
        "postgres://{}:{}@{}:{}/postgres",
        credentials.username, credentials.password, credentials.host, credentials.port
    ))
    .await?;

    let create_query = format!("CREATE DATABASE {}", credentials.database); // bindings didnt work?
    sqlx::query(&create_query).execute(&pool).await.unwrap();

    let pool = sqlx::PgPool::connect(&format!(
        "postgres://{}:{}@{}:{}/{}",
        credentials.username,
        credentials.password,
        credentials.host,
        credentials.port,
        credentials.database
    ))
    .await?;
    sqlx::migrate!().run(&pool).await.unwrap();

    Ok(credentials)
}
