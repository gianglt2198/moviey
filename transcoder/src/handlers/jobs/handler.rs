use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;

use crate::scheduler::executor::JobExecutor;

/// Get recent job executions
#[utoipa::path(
    get,
    path = "/api/jobs/executions",
    responses(
        (status = 200, description = "Job executions", body = serde_json::Value),
    ),
    tag = "Jobs"
)]
pub async fn get_job_executions(
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let executions = sqlx::query_as::<_, (String, String, i32, i32)>(
        r#"SELECT job_name, status, COALESCE(duration_seconds, 0), 
                  EXTRACT(EPOCH FROM (NOW() - completed_at))::INT as seconds_ago
           FROM job_executions
           WHERE completed_at IS NOT NULL
           ORDER BY completed_at DESC
           LIMIT 50"#,
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let items: Vec<_> = executions
        .into_iter()
        .map(|(name, status, duration, ago)| {
            json!({
                "job_name": name,
                "status": status,
                "duration_seconds": duration,
                "completed_seconds_ago": ago
            })
        })
        .collect();

    Ok(Json(json!({
        "total": items.len(),
        "executions": items
    })))
}

/// Get job execution history
#[utoipa::path(
    get,
    path = "/api/jobs/executions/{job_name}",
    params(("job_name" = String, Path, description = "Job name")),
    responses(
        (status = 200, description = "Job history", body = serde_json::Value),
    ),
    tag = "Jobs"
)]
pub async fn get_job_history(
    State(pool): State<Arc<PgPool>>,
    Path(job_name): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let executor = JobExecutor::new(pool.clone());
    let history = executor
        .get_job_history(&job_name, 20)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let items: Vec<_> = history
        .into_iter()
        .map(|exec| {
            json!({
                "id": exec.id,
                "status": format!("{:?}", exec.status),
                "started_at": exec.started_at.to_rfc3339(),
                "completed_at": exec.completed_at.map(|dt| dt.to_rfc3339()),
                "duration_seconds": exec.duration_seconds,
                "error": exec.error_message
            })
        })
        .collect();

    Ok(Json(json!({
        "job_name": job_name,
        "total_records": items.len(),
        "executions": items
    })))
}

/// Get job statistics
#[utoipa::path(
    get,
    path = "/api/jobs/statistics",
    responses(
        (status = 200, description = "Job statistics", body = serde_json::Value),
    ),
    tag = "Jobs"
)]
pub async fn get_job_statistics(
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let stats = sqlx::query_as::<_, (String, i32, i32, i32, i32)>(
        r#"SELECT job_name, total_runs, successful_runs, failed_runs, 
                  COALESCE(avg_duration_seconds, 0)
           FROM job_statistics
           ORDER BY last_run_at DESC NULLS LAST"#,
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let items: Vec<_> = stats
        .into_iter()
        .map(|(name, total, successful, failed, avg_duration)| {
            let success_rate = if total == 0 {
                0.0
            } else {
                (successful as f64 / total as f64) * 100.0
            };

            json!({
                "job_name": name,
                "total_runs": total,
                "successful_runs": successful,
                "failed_runs": failed,
                "success_rate_percent": format!("{:.1}%", success_rate),
                "avg_duration_seconds": avg_duration
            })
        })
        .collect();

    Ok(Json(json!({
        "total_jobs": items.len(),
        "statistics": items
    })))
}

/// Get batch job health status
#[utoipa::path(
    get,
    path = "/api/jobs/health",
    responses(
        (status = 200, description = "Job health", body = serde_json::Value),
    ),
    tag = "Jobs"
)]
pub async fn get_job_health(
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Check if critical jobs ran in last 24 hours
    let last_runs = sqlx::query_as::<_, (String, Option<i64>)>(
        r#"SELECT job_name, MAX(EXTRACT(EPOCH FROM (NOW() - completed_at)))::BIGINT
           FROM job_executions
           WHERE status = 'completed'
           GROUP BY job_name"#,
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut health_status = "healthy".to_string();
    let critical_jobs = vec!["cache_warming", "user_embeddings", "data_quality_checks"];

    for (job_name, last_run_secs) in &last_runs {
        if critical_jobs.contains(&job_name.as_str()) {
            let secs = last_run_secs.unwrap_or(i64::MAX);
            if secs > 86400 * 2 {
                // More than 2 days
                health_status = "degraded".to_string();
            }
        }
    }

    let last_failures: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM job_executions WHERE status = 'failed' AND started_at > NOW() - INTERVAL '24 hours'"
    )
    .fetch_one(pool.as_ref())
    .await
    .unwrap_or(0);

    Ok(Json(json!({
        "status": health_status,
        "last_failures_24h": last_failures,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}
