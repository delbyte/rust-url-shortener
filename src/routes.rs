use axum::{
    response::{IntoResponse, Response, Redirect, Html},
    routing::{get, post},
    Router, extract::{Path, Json, State, Query},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;
use rand::Rng;
use crate::db::Db;
use qrcodegen::QrCode;
use image::{Luma, ImageBuffer};
use base64::encode;
use std::io::Cursor;

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

#[derive(Deserialize)]
struct QrRequest {
    url: String,
}

pub fn create_router(db: Arc<SqlitePool>) -> Router {
    Router::new()
        .route("/", get(serve_index))
        .route("/shorten", post(shorten_url))
        .route("/:short_code", get(redirect_url))
        .route("/qr", get(generate_qr)) // QR Code route
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
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "URL must start with http:// or https://".to_string() })));
    }

    let existing_code = sqlx::query_as::<_, (String,)>("SELECT short_code FROM urls WHERE long_url = ?")
        .bind(&payload.long_url)
        .fetch_optional(&*db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("Database error: {}", e) })))?;

    if let Some((code,)) = existing_code {
        return Ok(Json(ShortenResponse { short_url: format!("https://flashurl-2u1k.onrender.com/{}", code) }));
    }

    let mut short_code = generate_short_code();

    while sqlx::query_as::<_, (String,)>("SELECT short_code FROM urls WHERE short_code = ?")
        .bind(&short_code)
        .fetch_optional(&*db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("Database error: {}", e) })))?.is_some() {
        short_code = generate_short_code();
    }

    sqlx::query("INSERT INTO urls (short_code, long_url) VALUES (?, ?)")
        .bind(&short_code)
        .bind(&payload.long_url)
        .execute(&*db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("Database error: {}", e) })))?;

    Ok(Json(ShortenResponse { short_url: format!("https://flashurl-2u1k.onrender.com/{}", short_code) }))
}

async fn redirect_url(
    State(db): State<Arc<SqlitePool>>,
    Path(short_code): Path<String>,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let result = sqlx::query_as::<_, (String,)>("SELECT long_url FROM urls WHERE short_code = ?")
        .bind(&short_code)
        .fetch_optional(&*db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("Database error: {}", e) })))?;

    match result {
        Some((long_url,)) => Ok(Redirect::permanent(&long_url).into_response()),
        None => Err((StatusCode::NOT_FOUND, Json(ErrorResponse { error: "Short code not found".to_string() }))),
    }
}

/// Generates a QR Code for the long URL and returns it as a base64-encoded PNG
async fn generate_qr(Query(params): Query<QrRequest>) -> Result<Json<String>, (StatusCode, Json<ErrorResponse>)> {
    let qr = QrCode::encode_text(&params.url, qrcodegen::QrCodeEcc::Medium)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "Failed to generate QR code".to_string() })))?;

    let original_size = qr.size();
    let scale_factor = 10; // Increase scale for higher resolution (higher values = higher resolution)
    let size = original_size * scale_factor; // Scaling QR code size for higher resolution
    let mut img = ImageBuffer::<Luma<u8>, Vec<u8>>::new(size as u32, size as u32);

    for y in 0..original_size {
        for x in 0..original_size {
            let color = if qr.get_module(x, y) { 0 } else { 255 };

            // Scale the QR code up
            for dy in 0..scale_factor {
                for dx in 0..scale_factor {
                    img.put_pixel(
                        (x * scale_factor + dx) as u32, 
                        (y * scale_factor + dy) as u32, 
                        Luma([color])
                    );
                }
            }
        }
    }

    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, image::ImageOutputFormat::Png)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "Failed to encode QR image".to_string() })))?;

    let base64_image = encode(buf.into_inner());

    Ok(Json(format!("data:image/png;base64,{}", base64_image)))
}



/// Generates a random 6-character alphanumeric short code
fn generate_short_code() -> String {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..6).map(|_| charset[rng.gen_range(0..charset.len())] as char).collect()
}
