use crate::{storage::db_connect::DbCredentials, webapp::webapp::Config};

pub fn get_config() -> Config {
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

    Config {
        port,
        db_credentials,
    }
}
