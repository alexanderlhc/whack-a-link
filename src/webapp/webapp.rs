use std::sync::Arc;

use axum::{serve::Serve, Router};
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;

use crate::config::AppConfig;
use crate::storage::db_connect;
use crate::webapp::webapp::db_connect::db_connect;

use super::routes::create_router;

pub struct WebApp {
    pub server: Serve<Router, Router>,
    pub port: String,
}

impl WebApp {
    pub async fn create_app(config: AppConfig) -> Result<WebApp, String> {
        let (server, port) = build(&config).await;

        Ok(WebApp {
            server,
            port: port.to_string(),
        })
    }
}

async fn build(config: &AppConfig) -> (Serve<Router, Router>, u16) {
    let tcp_listener = create_tcp_listener(&config.port).await.unwrap();
    let port = tcp_listener.local_addr().unwrap().port();
    let pool = db_connect(&config.db_credentials).await.unwrap();
    let app_state = create_app_state(&port.to_string(), pool);
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
    pub db: Pool<Postgres>,
}

fn create_app_state(port: &str, pool: Pool<Postgres>) -> Arc<AppState> {
    Arc::new(AppState {
        base_url: "http://localhost".to_string(),
        port: port.to_string(),
        db: pool,
    })
}

fn create_server(tcp_listener: TcpListener, router: Router) -> Serve<Router, Router> {
    axum::serve(tcp_listener, router)
}
