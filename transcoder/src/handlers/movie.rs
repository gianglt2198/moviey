use axum::Json;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::{
    extract::{Multipart, State},
    response::Result,
};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
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

pub async fn search_movies(  
    State(pool): State<PgPool>,  
    Query(params): Query<SearchQuery>,  
) -> Result<Json<Vec<MovieDetailDto>>> {  
    let mut query = String::from(  
        "SELECT id, title, status, duration_seconds, genre, director, release_year, rating, description  
         FROM movies WHERE status = 'completed'"  
    );

    // Add search filters  
    if let Some(search_term) = &params.q {  
        let safe_term = search_term.replace("'", "''");  
        query.push_str(&format!(  
            " AND (title ILIKE '%{}%' OR director ILIKE '%{}%' OR description ILIKE '%{}%')",  
            safe_term, safe_term, safe_term  
        ));  
    }  

    if let Some(genre_filter) = &params.genre {  
        let safe_genre = genre_filter.replace("'", "''");  
        query.push_str(&format!(" AND genre ILIKE '%{}%'", safe_genre));  
    }  

    // Add sorting  
    match params.sort.as_deref() {  
        Some("recent") => query.push_str(" ORDER BY created_at DESC"),  
        Some("rating") => query.push_str(" ORDER BY rating DESC"),  
        Some("title") => query.push_str(" ORDER BY title ASC"),  
        _ => query.push_str(" ORDER BY created_at DESC"),  
    }  

    let rows = sqlx::query(&query)  
        .fetch_all(&pool)  
        .await  
        .unwrap_or_default();  

    let movies = rows  
        .into_iter()  
        .map(|row| {  
            let title: String = row.get(1);  

            let rating = row.get::<Option<Decimal>, _>(7).map(|r| r.to_f64().unwrap_or(0.0));

            MovieDetailDto {  
                id: row.get(0),  
                title: title.clone(),  
                stream_url: format!(  
                    "http://localhost:3000/streams/{}/master.m3u8",  
                    title  
                ),  
                status: row.get(2),  
                duration: format!("{} min", row.get::<i32, _>(3) / 60),  
                thumbnail_url: format!(  
                    "http://localhost:3000/streams/{}/thumbnail.jpg",  
                    title  
                ),  
                genre: row.get(4),  
                director: row.get(5),  
                release_year: row.get(6),  
                rating: rating,  
                description: row.get(8),  
            }  
        })  
        .collect();  

    Ok(Json(movies))  
}  

pub async fn get_movie_detail(  
    State(pool): State<PgPool>,  
    Path(movie_id): Path<uuid::Uuid>,  
) -> Result<Json<MovieDetailDto>, StatusCode> {  
    let row = sqlx::query(  
        "SELECT id, title, status, duration_seconds, genre, director, release_year, rating, description   
         FROM movies WHERE id = $1"  
    )  
    .bind(movie_id)  
    .fetch_one(&pool)  
    .await  
    .map_err(|_| StatusCode::NOT_FOUND)?;  

    let title: String = row.get(1);  
    let movie = MovieDetailDto {  
        id: row.get(0),  
        title: title.clone(),  
        stream_url: format!(  
            "http://localhost:3000/streams/{}/master.m3u8",  
            title  
        ),  
        status: row.get(2),  
        duration: format!("{} min", row.get::<i32, _>(3) / 60),  
        thumbnail_url: format!(  
            "http://localhost:3000/streams/{}/thumbnail.jpg",  
            title  
        ),  
        genre: row.get(4),  
        director: row.get(5),  
        release_year: row.get(6),  
        rating: row.get(7),  
        description: row.get(8),  
    };  

    Ok(Json(movie))  
}  
