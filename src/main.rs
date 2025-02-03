mod routes;
mod db;

use axum::Router;
use tokio::net::TcpListener;
use db::init_db;
use std::sync::Arc;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let database_url = "sqlite://urls.db?mode=rwc";
    let db = Arc::new(init_db(database_url).await);

    let app = Router::new()
        .merge(routes::create_router(Arc::clone(&db)))
        .nest_service("/static", ServeDir::new("static"));

    // Bind to 0.0.0.0 to allow external access
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");

    // Start the server
    axum::serve(listener, app).await.unwrap();
}
