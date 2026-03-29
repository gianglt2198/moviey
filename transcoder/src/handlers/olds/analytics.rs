use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::*;
use crate::services::{analytics::AnalyticsService, validation::RuleValidator};

// pub(crate) fn router(pool: Arc<sqlx::PgPool>) -> OpenApiRouter {
//     OpenApiRouter::new()
//         .routes(routes!(save_enhanced_watch_progress))
//         .routes(routes!(get_completion_by_genre))
//         .routes(routes!(get_watch_patterns))
//         .routes(routes!(get_data_quality))
//         .routes(routes!(get_user_segment))
//         .with_state(pool)
// }

#[utoipa::path(
    post,
    path = "/api/analytics/watch-progress",
    responses(
        (status = 200, description = "Watch progress saved successfully"),
        (status = NOT_FOUND, description = "User profile not found")
    )
)]
pub async fn save_enhanced_watch_progress(
    claims: Claims,
    State(pool): State<PgPool>,
    Json(payload): Json<WatchProgressRequest>,
) -> Result<StatusCode> {
    let profile_id: Uuid = sqlx::query_scalar("SELECT id FROM profiles WHERE user_id = $1 LIMIT 1")
        .bind(claims.sub)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

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

    // Save watch history with additional metadata in analytics table
    AnalyticsService::save_watch_history_with_metadata(
        &pool,
        profile_id,
        payload,
        completion_percentage,
        playback_speed,
        completion_reason,
    )
    .await?;

    Ok(StatusCode::CREATED)
}

#[utoipa::path(
    get,
    path =  "/api/analytics/completion-by-genre",
    responses(
        (status = 200, description = "Completion rate by genre retrieved successfully"),
        (status = NOT_FOUND, description = "User profile not found")
    )
)]
// Get completion rate by genre
pub async fn get_completion_by_genre(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<CompletionRateByGenre>>> {
    let data = AnalyticsService::get_completion_rate_by_genre(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(data))
}

// Get watch patterns
#[utoipa::path(
    get,
    path =  "/api/analytics/watch-patterns",
    responses(
        (status = 200, description = "Watch patterns retrieved successfully"),
        (status = NOT_FOUND, description = "User profile not found")
    )
)]
pub async fn get_watch_patterns(
    _claims: Claims,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<WatchTimePattern>>> {
    let data = AnalyticsService::get_watch_patterns_by_time(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(data))
}

// Get data quality report
#[utoipa::path(
    get,
    path =  "/api/analytics/data-quality",
    responses(
        (status = 200, description = "Data quality report retrieved successfully"),
        (status = NOT_FOUND, description = "User profile not found")
    )
)]
pub async fn get_data_quality(State(pool): State<PgPool>) -> Result<Json<DataQualityReport>> {
    let report = AnalyticsService::get_data_quality_report(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(report))
}

// Get user's current segment
#[utoipa::path(
    get,
    path =  "/api/analytics/user/segment",
    responses(
        (status = 200, description = "User segment retrieved successfully"),
        (status = NOT_FOUND, description = "User profile not found")
    )
)]
pub async fn get_user_segment(
    claims: Claims,
    State(pool): State<PgPool>,
) -> Result<Json<UserSegment>, StatusCode> {
    let profile_id: Uuid = sqlx::query_scalar("SELECT id FROM profiles WHERE user_id = $1 LIMIT 1")
        .bind(claims.sub)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let segment = sqlx::query_as::<_, (Uuid, Uuid, String, f64, String, String, String)>(
        "SELECT id, profile_id, segment_type, segment_score,   
                last_calculated_at, valid_until, metadata FROM user_segments   
         WHERE profile_id = $1",
    )
    .bind(profile_id)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(UserSegment {
        id: segment.0,
        profile_id: segment.1,
        segment_type: segment.2,
        segment_score: segment.3,
        last_calculated_at: segment.4.parse().unwrap(),
        valid_until: segment.5.parse().unwrap(),
        metadata: serde_json::from_str(&segment.6).unwrap_or(serde_json::json!({})),
    }))
}
