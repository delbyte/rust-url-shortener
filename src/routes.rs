use axum::{
    routing::{get, post},
    Router, extract::{Path, Json, State}
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;
use rand::Rng;

type Db = Arc<SqlitePool>;

#[derive(Deserialize)]
struct ShortenRequest {
    long_url: String,
}

#[derive(Serialize)]
struct ShortenResponse {
    short_url: String,
}

// Initialize Router with SQLite
pub fn create_router(db: Db) -> Router {
    Router::new()
        .route("/shorten", post(shorten_url))
        .route("/:short_code", get(redirect_url))
        .with_state(db)
}

// Shorten URL and store in SQLite
async fn shorten_url(Json(payload): Json<ShortenRequest>, State(db): State<Db>) -> Json<ShortenResponse> {
    // Check if the URL already exists
    if let Some((existing_code,)) = sqlx::query_as::<_, (String,)>("SELECT short_code FROM urls WHERE long_url = ?")
        .bind(&payload.long_url)
        .fetch_optional(&*db)
        .await
        .unwrap()
    {
        return Json(ShortenResponse {
            short_url: format!("http://127.0.0.1:3000/{}", existing_code),
        });
    }

    let mut short_code = generate_short_code();

    // Ensure uniqueness by checking for collisions
    while sqlx::query_as::<_, (String,)>("SELECT short_code FROM urls WHERE short_code = ?")
        .bind(&short_code)
        .fetch_optional(&*db)
        .await
        .unwrap()
        .is_some()
    {
        short_code = generate_short_code();
    }

    sqlx::query("INSERT INTO urls (short_code, long_url) VALUES (?, ?)")
        .bind(&short_code)
        .bind(&payload.long_url)
        .execute(&*db)
        .await
        .unwrap();

    Json(ShortenResponse {
        short_url: format!("http://127.0.0.1:3000/{}", short_code),
    })
}

// Redirect handler
async fn redirect_url(Path(short_code): Path<String>, State(db): State<Db>) -> String {
    let result = sqlx::query_as::<_, (String,)>("SELECT long_url FROM urls WHERE short_code = ?")
        .bind(&short_code)
        .fetch_optional(&*db)
        .await
        .unwrap();

    match result {
        Some((long_url,)) => format!("Redirecting to: {}", long_url),
        None => "Short code not found!".to_string(),
    }
}

// Generate a random alphanumeric short code
fn generate_short_code() -> String {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..6).map(|_| charset[rng.gen_range(0..charset.len())] as char).collect()
}
