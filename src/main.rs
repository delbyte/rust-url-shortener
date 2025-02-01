mod routes;
mod db;

use axum::Router;
use tokio::net::TcpListener;
use db::init_db;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let database_url = "sqlite://urls.db";
    let db = init_db(database_url).await;

    let app = routes::create_router(db);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
