use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use rust_decimal::prelude::ToPrimitive;
use sqlx::PgPool;

use crate::domains::Movie;
use crate::dtos::favorites_dto::*;
use crate::models::Claims;

/// Toggle favorite status for a movie
#[utoipa::path(  
    post,  
    path = "/api/favorites/toggle",  
    request_body = ToggleFavoriteRequest,  
    security(  
        ("bearer_auth" = [])  
    ),  
    responses(  
        (status = 201, description = "Movie added to favorites"),  
        (status = 204, description = "Movie removed from favorites"),  
        (status = 401, description = "Unauthorized"),  
        (status = 500, description = "Internal server error")  
    ),  
    tag = "Favorites"  
)]  
pub async fn toggle_favorite(
    claims: Claims,
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<ToggleFavoriteRequest>,
) -> Result<StatusCode, StatusCode> {
    // Get or create profile
    let profile_id: Option<uuid::Uuid> =
        sqlx::query_scalar("SELECT id FROM profiles WHERE user_id = $1 LIMIT 1")
            .bind(claims.sub)
            .fetch_optional(pool.as_ref())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let profile_id = match profile_id {
        Some(id) => id,
        None => sqlx::query_scalar::<_, uuid::Uuid>(
            "INSERT INTO profiles (user_id, name) VALUES ($1, $2) RETURNING id",
        )
        .bind(claims.sub)
        .bind("Default Profile")
        .fetch_one(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    };

    // Check if already favorited
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM favorites WHERE profile_id = $1 AND movie_id = $2)",
    )
    .bind(profile_id)
    .bind(payload.movie_id)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if exists {
        // Remove favorite
        sqlx::query!(
            "DELETE FROM favorites WHERE profile_id = $1 AND movie_id = $2",
            profile_id,
            payload.movie_id
        )
        .execute(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(StatusCode::NO_CONTENT)
    } else {
        // Add favorite
        sqlx::query!(
            "INSERT INTO favorites (profile_id, movie_id) VALUES ($1, $2)",
            profile_id,
            payload.movie_id
        )
        .execute(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(StatusCode::CREATED)
    }
}

/// Get all favorites for current user
#[utoipa::path(  
    get,  
    path = "/api/favorites",  
    security(  
        ("bearer_auth" = [])  
    ),  
    responses(  
        (status = 200, description = "Favorites list retrieved", body = FavoritesListResponse),  
        (status = 401, description = "Unauthorized"),  
        (status = 404, description = "User profile not found")  
    ),  
    tag = "Favorites"  
)]  
pub async fn get_favorites(
    claims: Claims,
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<FavoritesListResponse>, StatusCode> {
    let profile_id: Option<uuid::Uuid> =
        sqlx::query_scalar("SELECT id FROM profiles WHERE user_id = $1 LIMIT 1")
            .bind(claims.sub)
            .fetch_optional(pool.as_ref())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let profile_id = profile_id.ok_or(StatusCode::NOT_FOUND)?;

    // Get favorite movies
    let rows: Vec<Movie> = sqlx::query_as(
        r#"SELECT m.* FROM  movies m 
           JOIN favorites f ON f.movie_id = m.id
           WHERE f.profile_id = $1 AND m.status = 'completed'
           ORDER BY f.created_at DESC"#,
    )
    .bind(profile_id)
    .fetch_all(pool.as_ref())
    .await
    .unwrap_or_default();

    let total_count = rows.len() as i32;
    let favorites = rows
        .into_iter()
        .map(|movie| FavoriteMovieResponse {
            id: movie.id,
            title: movie.title.clone(),
            stream_url: format!("http://localhost:3000/streams/{}/master.m3u8", movie.title),
            status: format!("{:?}", movie.status),
            duration: format!("{} min", movie.duration_seconds.unwrap_or(0) / 60),
            thumbnail_url: format!(
                "http://localhost:3000/streams/{}/thumbnail.jpg",
                movie.title
            ),
            genre: movie.genre,
            director: movie.director,
            release_year: movie.release_year,
            rating: movie.rating.map(|r| r.to_f64().unwrap_or(0.0)),
        })
        .collect();

    Ok(Json(FavoritesListResponse {
        total_count,
        favorites,
    }))
}
