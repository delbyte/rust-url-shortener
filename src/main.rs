use axum::{
    routing::{get, post},
    extract::State,
    response::{Html, IntoResponse},
    Router,
};
use sqlx::SqlitePool;
use std::{net::SocketAddr, sync::Arc};
use tower_http::services::ServeDir;

mod routes;

#[tokio::main]
async fn main() {
    // Connect to SQLite database
    let pool = SqlitePool::connect("sqlite:urls.db").await.unwrap();
    let db = Arc::new(pool);

    // Create Axum app with routes and static file serving
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static")) // Serve CSS & JS
        .route("/", get(serve_index)) // Serve the frontend
        .merge(routes::create_router(db)); // Include API routes

    // Define server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("\ud83d\ude80 Server running at http://{}", addr);

    // Start Axum server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Serve the HTML frontend
async fn serve_index() -> impl IntoResponse {
    Html(std::fs::read_to_string("templates/index.html").unwrap())
}