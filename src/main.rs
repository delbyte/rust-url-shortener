use axum::Router;
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;
use tokio::net::TcpListener;

mod routes;
use routes::create_router;

type Db = Arc<sqlx::SqlitePool>;

#[tokio::main]
async fn main() {
    let database_url = "sqlite://urls.db";
    let pool = SqlitePoolOptions::new()
        .connect(database_url)
        .await
        .expect("Failed to connect to database");
    
    sqlx::query("CREATE TABLE IF NOT EXISTS urls (short_code TEXT PRIMARY KEY, long_url TEXT UNIQUE)")
        .execute(&pool)
        .await
        .expect("Failed to create table");
    
    let db = Arc::new(pool);
    let app = create_router(db);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}