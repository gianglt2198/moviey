use std::sync::Arc;

use axum::extract::Path;
use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

use crate::config::redis::RedisPool;
use crate::domains::WatchHistory;
use crate::dtos::mappers::*;
use crate::dtos::watch_history_dto::*;
use crate::models::Claims;
use crate::services::cache::CacheInvalidation;
use crate::services::validation::RuleValidator;

/// Save watch progress
#[utoipa::path(  
    post,  
    path = "/api/watch-history/save",  
    request_body = SaveWatchProgressRequest,  
    security(  
        ("bearer_auth" = [])  
    ),  
    responses(  
        (status = 201, description = "Watch progress saved"),  
        (status = 400, description = "Invalid watch data"),  
        (status = 401, description = "Unauthorized")  
    ),  
    tag = "Watch History"  
)]  
pub async fn save_watch_progress(
    claims: Claims,
    State((pool, _redis)): State<(Arc<PgPool>, Arc<RedisPool>)>,  
    Json(payload): Json<SaveWatchProgressRequest>,
) -> Result<StatusCode, StatusCode> {
    // Get profile_id for this user
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

    sqlx::query!(
        r#"INSERT INTO watch_history (profile_id, movie_id, last_position_seconds, completed, watched_at, updated_at)
           VALUES ($1, $2, $3, $4, NOW(), NOW())
           ON CONFLICT(profile_id, movie_id)
           DO UPDATE SET last_position_seconds = $3, completed = $4, updated_at = NOW()"#,
        profile_id,
        payload.movie_id,
        payload.position_seconds,
        payload.completed
    )
    .execute(pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

/// Save enhanced watch progress with metadata
#[utoipa::path(  
    post,  
    path = "/api/watch-history/save-enhanced",  
    request_body = EnhancedWatchProgressRequest,  
    security(  
        ("bearer_auth" = [])  
    ),  
    responses(  
        (status = 201, description = "Enhanced watch data saved"),  
        (status = 400, description = "Invalid device type or watch data"),  
        (status = 401, description = "Unauthorized")  
    ),  
    tag = "Watch History"  
)]  
pub async fn save_enhanced_watch_progress(
    claims: Claims,
    State((pool, redis)): State<(Arc<PgPool>, Arc<RedisPool>)>,  
    Json(payload): Json<EnhancedWatchProgressRequest>,
) -> Result<StatusCode, StatusCode> {
    // Get profile_id
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

    // Validate data
    RuleValidator::validate_device_type(&payload.device_type)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let playback_speed = RuleValidator::validate_playback_speed(payload.playback_speed)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    RuleValidator::validate_interrupt_count(payload.interrupted_count)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Calculate completion percentage
    let completion_percentage = RuleValidator::calculate_completion_percentage(
        payload.watch_duration_seconds,
        payload.total_movie_duration_seconds,
    );

    let completion_reason =
        RuleValidator::determine_completion_reason(payload.completed, completion_percentage);

    // Save watch history with metadata
    sqlx::query(
        r#"INSERT INTO watch_history
           (profile_id, movie_id, watch_duration_seconds, total_movie_duration_seconds,
            completion_percentage, watch_quality, interrupted_count, playback_speed,
            device_type, completion_reason, watched_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW(), NOW())
        ON CONFLICT(profile_id, movie_id)
        DO UPDATE SET
            watch_duration_seconds = $3,
            total_movie_duration_seconds = $4,
            completion_percentage = $5,
            interrupted_count = $7,
            playback_speed = $8,
            device_type = $9,
            completion_reason = $10,
            updated_at = NOW()"#,
    )
    .bind(profile_id)
    .bind(payload.movie_id)
    .bind(payload.watch_duration_seconds)
    .bind(payload.total_movie_duration_seconds)
    .bind(completion_percentage)
    .bind(&payload.watch_quality)
    .bind(payload.interrupted_count)
    .bind(playback_speed)
    .bind(&payload.device_type)
    .bind(&completion_reason)
    .execute(pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // INVALIDATE CACHE  
    let mut invalidation = CacheInvalidation::new(redis.get_connection());  
    invalidation  
        .on_watch_event(profile_id, payload.movie_id)  
        .await  
        .ok(); // Log errors but don't fail the request  


    Ok(StatusCode::CREATED)
}

/// Get watch history for current user
#[utoipa::path(  
    get,  
    path = "/api/watch-history",  
    security(  
        ("bearer_auth" = [])  
    ),  
    responses(  
        (status = 200, description = "Watch history retrieved", body = Vec<WatchHistoryResponse>),  
        (status = 401, description = "Unauthorized"),  
        (status = 404, description = "User profile not found")  
    ),  
    tag = "Watch History"  
)]  
pub async fn get_watch_histories(
    claims: Claims,
    State((pool, _redis)): State<(Arc<PgPool>, Arc<RedisPool>)>,  
) -> Result<Json<Vec<WatchHistoryResponse>>, StatusCode> {
    let profile_id: Option<uuid::Uuid> =
        sqlx::query_scalar("SELECT id FROM profiles WHERE user_id = $1 LIMIT 1")
            .bind(claims.sub)
            .fetch_optional(pool.as_ref())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let profile_id = profile_id.ok_or(StatusCode::NOT_FOUND)?;

    let histories = sqlx::query_as::<_, WatchHistory>(
        "SELECT id, movie_id, last_position_seconds, completed, watched_at FROM watch_history WHERE profile_id = $1 ORDER BY updated_at DESC LIMIT 50",
    )
    .bind(profile_id)
    .fetch_all(pool.as_ref())
    .await
    .unwrap_or_default();

    let responses = histories
        .into_iter()
        .map(map_watch_history_to_response)
        .collect();

    Ok(Json(responses))
}

/// Get watch history for current user
#[utoipa::path(  
    get,  
    path = "/api/watch-history/{history_id}",  
    params(  
        ("history_id" = uuid::Uuid, Path, description = "Watch history entry ID")  
    ),  
    security(  
        ("bearer_auth" = [])  
    ),  
    responses(  
        (status = 200, description = "Watch history entry retrieved", body = WatchHistoryDetailResponse),  
        (status = 401, description = "Unauthorized"),  
        (status = 404, description = "History entry not found")  
    ),  
    tag = "Watch History"  
)]  
pub async fn get_watch_history(
    claims: Claims,
    State((pool, _redis)): State<(Arc<PgPool>, Arc<RedisPool>)>,  
    Path(history_id): Path<uuid::Uuid>,
) -> Result<Json<WatchHistoryDetailResponse>, StatusCode> {
    let profile_id: Option<uuid::Uuid> =
        sqlx::query_scalar("SELECT id FROM profiles WHERE user_id = $1 LIMIT 1")
            .bind(claims.sub)
            .fetch_optional(pool.as_ref())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let profile_id = profile_id.ok_or(StatusCode::NOT_FOUND)?;

    let history = sqlx::query_as::<_, WatchHistory>(
        "SELECT * FROM watch_history WHERE profile_id = $1 AND id = $2",
    )
    .bind(profile_id)
    .bind(history_id)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(map_watch_history_to_detail_response(history)))
}
