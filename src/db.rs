use sqlx::{Pool, Sqlite};
use std::sync::Arc;

pub type Db = Arc<Pool<Sqlite>>;

pub async fn init_db(database_url: &str) -> Db {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS urls (short_code TEXT PRIMARY KEY, long_url TEXT UNIQUE)",
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    Arc::new(pool)
}
