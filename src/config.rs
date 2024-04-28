use serde::Deserialize;
use thiserror::Error;

use crate::storage::db_connect::DbCredentials;

pub fn get_config() -> Result<AppConfig, ConfigError> {
    let c = config::Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .map_err(|e| ConfigError::FileError(e.to_string()))
        .unwrap();

    match c.try_deserialize() {
        Ok(config) => Ok(config),
        Err(e) => Err(ConfigError::ParseError(e.to_string())),
    }
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("File error: {0}")]
    FileError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub port: String,
    pub db_credentials: DbCredentials,
}
