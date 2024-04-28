use tracing::info;
use whack_a_link::{config::get_config, webapp::webapp::WebApp};

#[tokio::main]
async fn main() {
    init_logger();
    let config = get_config().unwrap();

    info!("Starting server: http://0.0.0.0:{}", config.port);

    let app = WebApp::create_app(config).await.unwrap();
    app.server.await.unwrap()
}

fn init_logger() {
    tracing_subscriber::fmt::init();
}
