use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use rust_decimal::{Decimal, prelude::ToPrimitive};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::*;

pub async fn rate_movie(
    claims: Claims,
    State(pool): State<PgPool>,
    Path(movie_id): Path<i32>,
    Json(payload): Json<RateMovieRequest>,
) -> Result<StatusCode, StatusCode> {
    if payload.rating < 0.5 || payload.rating > 5.0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let profile_id: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM profiles WHERE user_id = $1 LIMIT 1")
            .bind(claims.sub)
            .fetch_one(&pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let profile_id = match profile_id {
        Some(id) => id,
        None => sqlx::query_scalar::<_, uuid::Uuid>(
            "INSERT INTO profiles (user_id, name) VALUES ($1, $2) RETURNING id",
        )
        .bind(claims.sub)
        .bind("Default Profile")
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    };

    sqlx::query(
        r#"INSERT INTO user_ratings (profile_id, movie_id, rating) 
        VALUES ($1, $2, $3)
        ON CONFLICT (profile_id, movie_id) DO UPDATE SET rating = $3, rated_at = NOW()
        "#,
    )
    .bind(profile_id)
    .bind(movie_id)
    .bind(Decimal::from_f32_retain(payload.rating).unwrap_or_default())
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!(
        "⭐ User {} rated movie {} with {}/5.0",
        claims.sub, movie_id, payload.rating
    );

    Ok(StatusCode::OK)
}

pub async fn get_movie_rating(
    claims: Claims,
    State(pool): State<PgPool>,
    Path(movie_id): Path<i32>,
) -> Result<Json<Option<f32>>, StatusCode> {
    let profile_id: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM profiles WHERE user_id = $1 LIMIT 1")
            .bind(claims.sub)
            .fetch_one(&pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let profile_id = profile_id.ok_or(StatusCode::NOT_FOUND)?;

    let rating: Option<Decimal> = sqlx::query_scalar(
        "SELECT rating FROM user_ratings WHERE profile_id = $1 AND movie_id = $2",
    )
    .bind(profile_id)
    .bind(movie_id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rating.map(|r| r.to_f32().unwrap_or(0.0))))
}
