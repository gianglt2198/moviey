use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::redis::RedisPool,
    dtos::*,
    models::Claims,
    services::{cache::RecommendationCache, hybrid_recommender::HybridRecommender},
};


/// Generate personalized recommendations for a user  
#[utoipa::path(  
    post,  
    path = "/api/recommendations/generate/{user_id}",  
    params(("user_id" = Uuid, Path, description = "User ID")),  
    request_body = GenerateRecommendationsRequest,  
    security(("bearer_auth" = [])),  
    responses(  
        (status = 200, description = "Recommendations generated", body = RecommendationsListResponse),  
        (status = 401, description = "Unauthorized"),  
        (status = 404, description = "User not found"),  
        (status = 500, description = "Internal server error")  
    ),  
    tag = "Recommendations"  
)]  
pub async fn generate_recommendations(
    _claims: Claims,
      State((pool, redis)): State<(Arc<PgPool>, Arc<RedisPool>)>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<GenerateRecommendationsRequest>,
) -> Result<Json<RecommendationsListResponse>, StatusCode> {
    let profile_id: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM profiles WHERE user_id = $1 LIMIT 1")
            .bind(user_id)
            .fetch_optional(pool.as_ref())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let profile_id = profile_id.ok_or(StatusCode::NOT_FOUND)?;

    let limit = payload.limit.unwrap_or(10).min(50);
    let diversity = payload.diversity_factor.unwrap_or(0.5).clamp(0.0, 1.0);

    let mut cache = RecommendationCache::new(redis.get_connection());  

        // Try to get from cache first  
    if let Ok(Some(cached)) = cache.get_recommendations(profile_id).await {  
        cache.increment_hit().await.ok();  
        
        let response = RecommendationsListResponse {  
            recommendations: cached.recommendations[..limit as usize].to_vec(),  
            total: cached.recommendations.len() as i32,  
            generated_at: cached.generated_at,  
            expires_at: cached.expires_at,  
        };  

        return Ok(Json(response));  
    }  

     // Cache miss - generate recommendations  
    cache.increment_miss().await.ok();  


    let recommender = HybridRecommender::default();
    let recommendations = recommender
        .generate_recommendation(pool.as_ref(), profile_id, limit, diversity)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let responses: Vec<RecommendationResponse> = recommendations
        .iter()
        .enumerate()
        .map(|(idx, (movie_id, score, reason))| RecommendationResponse {
            movie_id: *movie_id,
            title: "Movie Title".to_string(),
            score: *score,
            reason: reason.clone(),
            rank: (idx + 1) as i32,
            thumbnail_url: format!("http://localhost:3000/streams/{}/thumbnail.jpg", movie_id),
            genre: None,
        })
        .collect();

         // Cache the results  
    cache.set_recommendations(profile_id, responses.clone()).await.ok();  

    Ok(Json(RecommendationsListResponse {
        recommendations: responses,
        total: recommendations.len() as i32,
        generated_at: chrono::Utc::now().to_rfc3339(),
        expires_at: (chrono::Utc::now() + chrono::Duration::hours(24)).to_rfc3339(),
    }))
}


/// Get cached recommendations  
#[utoipa::path(  
    get,  
    path = "/api/recommendations/{user_id}",  
    params(("user_id" = Uuid, Path, description = "User ID")),  
    security(("bearer_auth" = [])),  
    responses(  
        (status = 200, description = "Cached recommendations", body = RecommendationsListResponse),  
        (status = 404, description = "Recommendations not found")  
    ),  
    tag = "Recommendations"  
)]  
pub async fn get_recommendations(  
    claims: Claims,  
    Path(user_id): Path<Uuid>,  
    State((pool, redis)): State<(Arc<PgPool>, Arc<RedisPool>)>,  
) -> Result<Json<RecommendationsListResponse>, StatusCode> {  
    // TODO: Fetch from Redis cache first  
    // If miss, generate and cache  
    Err(StatusCode::NOT_FOUND)  
}  

/// Get similar movies  
#[utoipa::path(  
    get,  
    path = "/api/recommendations/similar/{movie_id}",  
    params(("movie_id" = Uuid, Path, description = "Movie ID")),  
    responses(  
        (status = 200, description = "Similar movies", body = Vec<SimilarMovieResponse>),  
        (status = 404, description = "Movie not found")  
    ),  
    tag = "Recommendations"  
)]  
pub async fn get_similar_movies(  
    Path(movie_id): Path<Uuid>,  
    State((pool, redis)): State<(Arc<PgPool>, Arc<RedisPool>)>,  
) -> Result<Json<Vec<SimilarMovieResponse>>, StatusCode> {  
    // TODO: Implement  
    Ok(Json(vec![]))  
}  


/// Log recommendation feedback  
#[utoipa::path(  
    post,  
    path = "/api/recommendations/feedback",  
    request_body = RecommendationFeedbackRequest,  
    security(("bearer_auth" = [])),  
    responses(  
        (status = 201, description = "Feedback recorded"),  
        (status = 500, description = "Internal server error")  
    ),  
    tag = "Recommendations"  
)]  
pub async fn save_recommendation_feedback(  
    _claims: Claims,  
    State((pool, redis)): State<(Arc<PgPool>, Arc<RedisPool>)>,  
    Json(payload): Json<RecommendationFeedbackRequest>,  
) -> Result<StatusCode, StatusCode> {  
    // TODO: Implement feedback logging  
    Ok(StatusCode::CREATED)  
}  


/// Get cache statistics  
#[utoipa::path(  
    get,  
    path = "/api/recommendations/cache/stats",  
    responses(  
        (status = 200, description = "Cache statistics", body = serde_json::Value),  
    ),  
    tag = "Recommendations"  
)]  
pub async fn get_cache_stats(  
    State((_pool, redis)): State<(Arc<PgPool>, Arc<RedisPool>)>,  
) -> Result<Json<serde_json::Value>, StatusCode> {  
    let mut cache = RecommendationCache::new(redis.get_connection());  
    let stats = cache.get_cache_stats().await  
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;  

    Ok(Json(serde_json::json!({  
        "hits": stats.hits,  
        "misses": stats.misses,  
        "total_requests": stats.total_requests,  
        "hit_rate": format!("{:.2}%", stats.hit_rate),  
        "invalidations": stats.invalidations,  
    })))  
}  