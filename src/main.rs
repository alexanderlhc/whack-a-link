use whack_a_link::{
    storage::db_connect::DbCredentials,
    webapp::webapp::{Config, WebApp},
};

#[tokio::main]
async fn main() {
    // Database
    let db_credentials = DbCredentials {
        database: "test".to_string(),
        username: "db_user".to_string(),
        password: "password".to_string(),
        host: "localhost".to_string(),
        port: "5432".to_string(),
    };

    // WebServer
    let port = "8000".to_string();
    let config = Config {
        port,
        db_credentials,
    };

    println!("Starting server: http://0.0.0.0:{}", config.port);

    let app = WebApp::create_app(config).await.unwrap();
    app.server.await.unwrap()
}
