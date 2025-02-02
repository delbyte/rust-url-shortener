use axum::{
    response::{IntoResponse, Response, Redirect, Html},
    routing::{get, post},
    Router, extract::{Path, Json, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;
use rand::Rng;
use crate::db::Db;

#[derive(Deserialize)]
struct ShortenRequest {
    long_url: String,
}

#[derive(Serialize)]
struct ShortenResponse {
    short_url: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub fn create_router(db: Arc<SqlitePool>) -> Router {
    Router::new()
        .route("/", get(serve_index))
        .route("/shorten", post(shorten_url))
        .route("/:short_code", get(redirect_url))
        .with_state(db)
}

async fn serve_index() -> impl IntoResponse {
    Html(include_str!("../static/index.html"))
}

async fn shorten_url(
    State(db): State<Arc<SqlitePool>>,
    Json(payload): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>, (StatusCode, Json<ErrorResponse>)> {
    if !payload.long_url.starts_with("http://") && !payload.long_url.starts_with("https://") {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "URL must start with http:// or https://".to_string(),
            }),
        ));
    }

    let existing_code = sqlx::query_as::<_, (String,)>("SELECT short_code FROM urls WHERE long_url = ?")
        .bind(&payload.long_url)
        .fetch_optional(&*db)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Database error: {}", e),
            }),
        ))?;

    if let Some((code,)) = existing_code {
        return Ok(Json(ShortenResponse {
            short_url: format!("http://127.0.0.1:3000/{}", code),
        }));
    }

    let mut short_code = generate_short_code();

    while sqlx::query_as::<_, (String,)>("SELECT short_code FROM urls WHERE short_code = ?")
        .bind(&short_code)
        .fetch_optional(&*db)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Database error: {}", e),
            }),
        ))?.is_some() {
        short_code = generate_short_code();
    }

    sqlx::query("INSERT INTO urls (short_code, long_url) VALUES (?, ?)")
        .bind(&short_code)
        .bind(&payload.long_url)
        .execute(&*db)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Database error: {}", e),
            }),
        ))?;

    Ok(Json(ShortenResponse {
        short_url: format!("http://127.0.0.1:3000/{}", short_code),
    }))
}

async fn redirect_url(
    State(db): State<Arc<SqlitePool>>,
    Path(short_code): Path<String>,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let result = sqlx::query_as::<_, (String,)>("SELECT long_url FROM urls WHERE short_code = ?")
        .bind(&short_code)
        .fetch_optional(&*db)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Database error: {}", e),
            }),
        ))?;

    match result {
        Some((long_url,)) => Ok(Redirect::permanent(&long_url).into_response()),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Short code not found".to_string(),
            }),
        )),
    }
}

fn generate_short_code() -> String {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..6).map(|_| charset[rng.gen_range(0..charset.len())] as char).collect()
}
