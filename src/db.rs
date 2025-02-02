use sqlx::{Pool, Sqlite};
use std::sync::Arc;

pub type Db = Arc<Pool<Sqlite>>;

pub async fn init_db(database_url: &str) -> Pool<Sqlite> {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(database_url)
        .await
        .unwrap_or_else(|_| panic!("Failed to connect to database at {}. Check path or permissions.", database_url));

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS urls (short_code TEXT PRIMARY KEY, long_url TEXT UNIQUE)",
    )
    .execute(&pool)
    .await
    .expect("Failed to create table. Check database file path and permissions.");

    pool
}
