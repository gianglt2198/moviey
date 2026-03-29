use sqlx::PgPool;

use crate::dtos::analytics_dto::*;

pub struct AnalyticsService;

impl AnalyticsService {
    pub async fn get_completion_rate_by_genre(
        pool: &PgPool,
    ) -> Result<Vec<CompletionRateByGenre>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (String, i64, f64, f64)>(
            r#"SELECT 
                m.genre,  
                COUNT(*) as total_watches,  
                AVG(wh.completion_percentage) as avg_completion_rate,  
                PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY wh.completion_percentage) as median  
                FROM watch_history wh  
               JOIN movies m ON wh.movie_id = m.id
               WHERE wh.created_at > NOW() - INTERVAL '30 days'
               GROUP BY m.genre
               ORDER BY avg_completion_rate DESC"#,
        )
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(
                |(genre, total_watches, avg_completion_rate, median_completion)| {
                    CompletionRateByGenre {
                        genre,
                        total_watches,
                        avg_completion_rate,
                        median_completion,
                    }
                },
            )
            .collect())
    }

    pub async fn get_watch_patterns_by_time(
        pool: &PgPool,
    ) -> Result<Vec<WatchTimePattern>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (String, i64, f64)>(
            r#"SELECT 
                CASE   
                    WHEN EXTRACT(HOUR FROM wh.watched_at) BETWEEN 6 AND 11 THEN 'morning'  
                    WHEN EXTRACT(HOUR FROM wh.watched_at) BETWEEN 12 AND 16 THEN 'afternoon'  
                    WHEN EXTRACT(HOUR FROM wh.watched_at) BETWEEN 17 AND 20 THEN 'evening'  
                    ELSE 'night'  
                END as time_period, 
                COUNT(*) as watch_count,  
                AVG(wh.completion_percentage) as avg_completion  
               FROM watch_history wh  
               WHERE wh.created_at > NOW() - INTERVAL '30 days'
               GROUP BY time_period
              ORDER BY watch_count DESC"#,
        )
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(
                |(time_period, watch_count, avg_completion)| WatchTimePattern {
                    time_period,
                    watch_count,
                    avg_completion,
                },
            )
            .collect())
    }

    pub async fn get_data_quality_report(pool: &PgPool) -> Result<DataQualityReport, sqlx::Error> {
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM watch_history")
            .fetch_one(pool)
            .await?;

        let flagged: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM watch_history WHERE flagged_for_review = true",
        )
        .fetch_one(pool)
        .await?;

        let top_issues: Vec<(String, i64)> = sqlx::query_as(
            "SELECT flag_type, COUNT(*) as count FROM data_quality_flags   
             WHERE resolved = false   
             GROUP BY flag_type   
             ORDER BY count DESC   
             LIMIT 5",
        )
        .fetch_all(pool)
        .await?;

        Ok(DataQualityReport {
            total_records: total,
            valid_records: total - flagged,
            flagged_records: flagged,
            validation_percentage: if total > 0 {
                ((total - flagged) as f64 / total as f64) * 100.0
            } else {
                0.0
            },
            top_issues,
        })
    }
}
