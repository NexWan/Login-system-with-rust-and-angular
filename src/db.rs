use sqlx::postgres::PgPoolOptions;


pub async fn get_connection() -> Result<sqlx::PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:test@172.17.0.1:5432/test")
        .await?;
    Ok(pool)
}