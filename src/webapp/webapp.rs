use axum::{serve::Serve, Router};
use tokio::net::TcpListener;

use super::routes::create_router;

pub struct Config {
    pub port: String,
}

pub struct WebApp {
    pub server: Serve<Router, Router>,
    pub port: String,
}

impl WebApp {
    pub async fn create_app(config: Config) -> Result<WebApp, String> {
        let app = build(&config).await;
        Ok(WebApp {
            server: app,
            port: config.port,
        })
    }
}

async fn build(config: &Config) -> Serve<Router, Router> {
    let router = create_router();
    let tcp_listener = create_tcp_listener(&config.port).await.unwrap();
    let server = create_server(tcp_listener, router);
    server
}

async fn create_tcp_listener(port: &str) -> Result<TcpListener, String> {
    let tcp_listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await;
    match tcp_listener {
        Ok(tcp_listener) => Ok(tcp_listener),
        Err(_) => Err("ERROR".to_string()),
    }
}

fn create_server(tcp_listener: TcpListener, router: Router) -> Serve<Router, Router> {
    axum::serve(tcp_listener, router)
}
