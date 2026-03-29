use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

use crate::dtos::*;
use crate::models::Claims;
use crate::services::analytics::AnalyticsService;

/// Get completion rate by genre
#[utoipa::path(  
    get,  
    path = "/api/analytics/completion-by-genre",  
    responses(  
        (status = 200, description = "Genre completion rates retrieved", body = Vec<CompletionRateByGenre>),  
        (status = 500, description = "Internal server error")  
    ),  
    tag = "Analytics"  
)]  
pub async fn get_completion_by_genre(
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<Vec<CompletionRateByGenre>>, StatusCode> {
    let data = AnalyticsService::get_completion_rate_by_genre(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(data))
}

/// Get watch patterns by time of day
#[utoipa::path(  
    get,  
    path = "/api/analytics/watch-patterns",  
    responses(  
        (status = 200, description = "Watch patterns retrieved", body = Vec<WatchTimePattern>),  
        (status = 500, description = "Internal server error")  
    ),  
    tag = "Analytics"  
)]  
pub async fn get_watch_patterns(
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<Vec<WatchTimePattern>>, StatusCode> {
    let data = AnalyticsService::get_watch_patterns_by_time(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(data))
}

/// Get data quality report
#[utoipa::path(  
    get,  
    path = "/api/analytics/data-quality",  
    responses(  
        (status = 200, description = "Data quality report retrieved", body = DataQualityReport),  
        (status = 500, description = "Internal server error")  
    ),  
    tag = "Analytics"  
)]  
pub async fn get_data_quality(
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<DataQualityReport>, StatusCode> {
    let report = AnalyticsService::get_data_quality_report(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(report))
}

/// Get user's segment
#[utoipa::path(  
    get,  
    path = "/api/analytics/user/segment",  
    security(  
        ("bearer_auth" = [])  
    ),  
    responses(  
        (status = 200, description = "User segment retrieved", body = UserSegment),  
        (status = 401, description = "Unauthorized"),  
        (status = 404, description = "User or segment not found")  
    ),  
    tag = "Analytics"  
)]  
pub async fn get_user_segment(
    claims: Claims,
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<UserSegment>, StatusCode> {
    let profile_id: Option<uuid::Uuid> =
        sqlx::query_scalar("SELECT id FROM profiles WHERE user_id = $1 LIMIT 1")
            .bind(claims.sub)
            .fetch_optional(pool.as_ref())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let profile_id = profile_id.ok_or(StatusCode::NOT_FOUND)?;

    let segment = sqlx::query_as::<
        _,
        (
            uuid::Uuid,
            uuid::Uuid,
            String,
            f64,
            String,
            String,
            serde_json::Value,
        ),
    >(
        r#"SELECT id, profile_id, segment_type, segment_score,
                  last_calculated_at, valid_until, metadata FROM user_segments
           WHERE profile_id = $1"#,
    )
    .bind(profile_id)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(UserSegment {
        id: segment.0,
        profile_id: segment.1,
        segment_type: segment.2,
        segment_score: segment.3,
        last_calculated_at: segment.4.parse().unwrap_or_else(|_| chrono::Utc::now()),
        valid_until: segment.5.parse().unwrap_or_else(|_| chrono::Utc::now()),
        metadata: segment.6,
    }))
}
