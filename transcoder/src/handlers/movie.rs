use axum::Json;
use axum::http::StatusCode;
use axum::{
    extract::{Multipart, State},
    response::Result,
};
use sqlx::{PgPool, Row};

use crate::models::*;

pub async fn get_movies(State(pool): State<PgPool>) -> Result<Json<Vec<MovieDto>>> {
    let rows = sqlx::query(
        "SELECT id, title, status, duration_seconds FROM movies WHERE status = 'completed' ORDER BY created_at DESC",
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let movies = rows
        .into_iter()
        .map(|row| MovieDto {
            id: row.get(0),
            title: row.get(1),
            stream_url: format!(
                "http://localhost:3000/streams/{}/master.m3u8",
                row.get::<String, _>(1)
            ),
            status: row.get(2),
            duration: format!("{} min", row.get::<i32, _>(3) / 60),
            thumbnail_url: format!(
                "http://localhost:3000/streams/{}/thumbnail.jpg",
                row.get::<String, _>(1)
            ),
        })
        .collect();

    Ok(axum::Json(movies))
}

pub async fn upload_movie(
    State(_pool): State<PgPool>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, StatusCode> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        let filename = field
            .file_name()
            .ok_or(StatusCode::BAD_REQUEST)?
            .to_string();

        let bytes = field
            .bytes()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Save file
        let path = format!("./uploads/{}", filename);
        std::fs::write(&path, bytes).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        println!("📤 Uploaded: {}", filename);
    }

    Ok(Json(serde_json::json!({"message": "Upload successful"})))
}
