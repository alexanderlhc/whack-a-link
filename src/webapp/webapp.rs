use std::sync::Arc;

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
        let (server, port) = build(&config).await;

        Ok(WebApp {
            server,
            port: port.to_string(),
        })
    }
}

async fn build(config: &Config) -> (Serve<Router, Router>, u16) {
    let tcp_listener = create_tcp_listener(&config.port).await.unwrap();
    let port = tcp_listener.local_addr().unwrap().port();
    let app_state = create_app_state(&port.to_string());
    let router = create_router(app_state);
    let server = create_server(tcp_listener, router);
    (server, port)
}

async fn create_tcp_listener(port: &str) -> Result<TcpListener, String> {
    let tcp_listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await;
    match tcp_listener {
        Ok(tcp_listener) => Ok(tcp_listener),
        Err(_) => Err("ERROR".to_string()),
    }
}

pub struct AppState {
    pub base_url: String,
    pub port: String,
}

fn create_app_state(port: &str) -> Arc<AppState> {
    Arc::new(AppState {
        base_url: "http://localhost".to_string(),
        port: port.to_string(),
    })
}

fn create_server(tcp_listener: TcpListener, router: Router) -> Serve<Router, Router> {
    axum::serve(tcp_listener, router)
}
