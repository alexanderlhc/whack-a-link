use sqlx::{Pool, Postgres};

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

    migrate_db(&client).await?;

    Ok(client)
}

async fn migrate_db(pool: &Pool<Postgres>) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
