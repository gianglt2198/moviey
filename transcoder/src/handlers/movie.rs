use axum::Json;
use axum::{extract::State, response::Result};
use sqlx::Row;

use crate::models::*;

pub async fn get_movies(State(pool): State<sqlx::PgPool>) -> Result<Json<Vec<MovieDto>>> {
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
