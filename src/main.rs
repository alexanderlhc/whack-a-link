use whack_a_link::{
    storage::{
        db_connect::{db_connect, DbCredentials},
        storage_shortcode::get_url_by_shortcode,
    },
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

    let pool = db_connect(&db_credentials).await.unwrap();
    let url = get_url_by_shortcode("acdbc", &pool).await.unwrap();
    if let Some(url) = url {
        println!("URL: {}", url.url);
        println!("Shortcode: {}", url.shortcode);
    } else {
        println!("URL not found");
    }

    // let pool = match db_connect(&db_credentials).await {
    //     Ok(_) => println!("Connected to database"),
    //     Err(e) => {
    //         println!("Failed to connect to database: {}", e);
    //         return;
    //     }
    // }

    // WebServer
    let port = "8000".to_string();
    let config = Config { port };

    println!("Starting server: http://0.0.0.0:{}", config.port);

    let app = WebApp::create_app(config).await.unwrap();
    app.server.await.unwrap()
}
