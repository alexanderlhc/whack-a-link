use serde::Deserialize;
use sqlx::{Pool, Postgres};
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct DbCredentials {
    pub database: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
}

pub async fn db_connect(
    credentials: &DbCredentials,
) -> Result<Pool<Postgres>, Box<dyn std::error::Error>> {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        credentials.username,
        credentials.password,
        credentials.host,
        credentials.port,
        credentials.database
    );
    let client = sqlx::PgPool::connect(&url).await?;

    info!("connected to database: {}", credentials.database);

    migrate_db(&client).await?;

    Ok(client)
}

async fn migrate_db(pool: &Pool<Postgres>) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::migrate!("./migrations").run(pool).await?;
    info!("db migrations ran successfully");
    Ok(())
}
