use reqwest::Response;
use whack_a_link::webapp::webapp::{Config, WebApp};

pub struct TestWebApp {
    pub address: String,
}

pub async fn run_server() -> TestWebApp {
    let port = "0".to_string(); // 0 assigns random port
    let config = Config { port };
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
}
