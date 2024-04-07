#[derive(sqlx::FromRow)]
pub struct ShortcodeRow {
    pub shortcode: String,
    pub url: String,
}

pub async fn get_url_by_shortcode(
    shortcode: &str,
    pool: &sqlx::PgPool,
) -> Result<Option<ShortcodeRow>, Box<dyn std::error::Error>> {
    let row = sqlx::query_as!(
        ShortcodeRow,
        r#"
        SELECT 
          shortcode,
          url
        FROM links
        WHERE shortcode = $1
        "#,
        shortcode
    )
    .fetch_one(pool)
    .await;

    match row {
        Ok(row) => Ok(Some(row)),
        Err(_) => Ok(None),
    }
}
