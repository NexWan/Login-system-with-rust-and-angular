use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
/*
This module is used to create a connection to the database.
The connection is created using the PgPoolOptions struct from the sqlx::postgres module.
The max_connections method is used to set the maximum number of connections to the database.
The connect method is used to establish a connection to the database using the connection string.
The connection string is in the format postgres://username:password@host:port/database.
*/

pub async fn get_connection() -> Result<sqlx::PgPool, sqlx::Error> {
    dotenv().ok();
    let url = std::env::var("URL_CONNECTION").expect("URL_CONNECTION must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;
    Ok(pool)
}
