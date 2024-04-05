use whack_a_link::webapp::webapp::{Config, WebApp};

#[tokio::main]
async fn main() {
    let port = "8080".to_string();
    let config = Config { port };

    println!("Starting server: http://0.0.0.0:{}", config.port);

    let app = WebApp::create_app(config).await.unwrap();
    app.server.await.unwrap()
}
