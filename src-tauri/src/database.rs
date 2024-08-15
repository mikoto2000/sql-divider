use std::env;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

const DATABASE_URL_DEFAULT: &str = "postgres://postgres:postgres@localhost/postgres";

pub async fn create_connection_pool() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").unwrap_or(DATABASE_URL_DEFAULT.to_string());

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap()
}
