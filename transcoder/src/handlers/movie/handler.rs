use std::sync::Arc;

use axum::{
    Json,
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{domains::*, dtos::*};

/// Get all movies (paginated, completed only)  
#[utoipa::path(  
    get,  
    path = "/api/movies",  
    responses(  
        (status = 200, description = "List of movies retrieved successfully", body = Vec<MovieResponse>),  
        (status = 500, description = "Internal server error")  
    ),  
    tag = "Movies"  
)]  
pub async fn get_movies(
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<Vec<MovieResponse>>, StatusCode> {
    let movies = sqlx::query_as::<_, Movie>(
        "SELECT * FROM movies WHERE status = 'completed' ORDER BY created_at DESC LIMIT 20",
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let responses = movies.into_iter().map(map_movie_to_response).collect();

    Ok(Json(responses))
}

/// Search movies by title, director, or description  
#[utoipa::path(  
    get,  
    path = "/api/movies/search",  
    params(SearchMovieQuery),  
    responses(  
        (status = 200, description = "Search results retrieved", body = Vec<MovieDetailResponse>),  
        (status = 400, description = "Invalid search parameters"),  
        (status = 500, description = "Internal server error")  
    ),  
    tag = "Movies"  
)] 
pub async fn search_movies(
    State(pool): State<Arc<PgPool>>,
    Query(params): Query<SearchMovieQuery>,
) -> Result<Json<Vec<MovieDetailResponse>>, StatusCode> {
    let mut query = String::from("SELECT * FROM movies WHERE status = 'completed'");

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

    let movies = sqlx::query_as::<_, Movie>(&query)
        .fetch_all(pool.as_ref())
        .await
        .unwrap_or_default();

    let responses = movies
        .into_iter()
        .map(map_movie_to_detail_response)
        .collect();

    Ok(Json(responses))
}

/// Get movie details by ID  
#[utoipa::path(  
    get,  
    path = "/api/movies/{movie_id}",  
    params(  
        ("movie_id" = Uuid, Path, description = "Unique movie identifier")  
    ),  
    responses(  
        (status = 200, description = "Movie details retrieved", body = MovieDetailResponse),  
        (status = 404, description = "Movie not found"),  
        (status = 500, description = "Internal server error")  
    ),  
    tag = "Movies"  
)]
pub async fn get_movie_detail(
    State(pool): State<Arc<PgPool>>,
    Path(movie_id): Path<Uuid>,
) -> Result<Json<MovieDetailResponse>, StatusCode> {
    let movie = sqlx::query_as::<_, Movie>("SELECT * FROM movies WHERE id = $1")
        .bind(movie_id)
        .fetch_one(pool.as_ref())
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(map_movie_to_detail_response(movie)))
}

/// Upload a new movie file  
#[utoipa::path(  
    post,  
    path = "/api/movies/upload",  
    responses(  
        (status = 200, description = "Movie uploaded successfully", body = serde_json::Value),  
        (status = 400, description = "Invalid file upload"),  
        (status = 500, description = "Upload processing failed")  
    ),  
    tag = "Movies"  
)]  
pub async fn upload_movie(
    State(_pool): State<Arc<PgPool>>,
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

        // TODO: add upload folder to config later
        let path = format!("./uploads/{}", filename);
        std::fs::write(&path, bytes).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        println!("📤 Uploaded: {}", filename);
    }

    Ok(Json(serde_json::json!({"message": "Upload successful"})))
}
