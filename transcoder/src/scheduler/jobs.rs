use crate::config::redis::RedisPool;
use crate::scheduler::{JobExecutor};
use crate::services::cache::warming::CacheWarmer;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct BatchJobs;

impl BatchJobs {
    /// Job 1: Calculate User Embeddings - 2 AM UTC daily
    pub async fn calculate_user_embeddings(
        pool: Arc<PgPool>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let executor = JobExecutor::new(pool.clone());
        let job_name = "calculate_user_embeddings";
        let job_id = executor.start_job(job_name).await?;

        match Self::_calculate_user_embeddings_impl(pool.as_ref()).await {
            Ok(count) => {
                executor.complete_job(job_id, job_name).await?;
                println!("📊 Updated embeddings for {} users", count);
                Ok(())
            }
            Err(e) => {
                executor.fail_job(job_id, job_name, &e.to_string()).await?;
                Err(e)
            }
        }
    }

    async fn _calculate_user_embeddings_impl(
        pool: &PgPool,
    ) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        // Get all active users
        let users: Vec<Uuid> = sqlx::query_scalar(
            r#"SELECT DISTINCT profile_id FROM watch_history   
               WHERE watched_at > NOW() - INTERVAL '90 days'  
               LIMIT 10000"#,
        )
        .fetch_all(pool)
        .await?;

        let mut count = 0;

        for profile_id in users {
            let watches: Vec<(Uuid, f64)> = sqlx::query_as::<_, (Uuid, f64)>(
                r#"SELECT movie_id, rating FROM watch_history   
                   WHERE profile_id = $1 AND watched_at > NOW() - INTERVAL '90 days'"#,
            )
            .bind(profile_id)
            .fetch_all(pool)
            .await?;

            if watches.is_empty() {
                continue; // Skip users with no recent activity
            }

            // Get movie features
            let mut genre_weights = std::collections::HashMap::new();
            let mut director_weights = std::collections::HashMap::new();
            let mut total_weight = 0.0;

            for (movie_id, rating) in &watches {
                let (genre, director): (String, String) =
                    sqlx::query_as("SELECT genre, director FROM movies WHERE id = $1")
                        .bind(movie_id)
                        .fetch_optional(pool)
                        .await?
                        .unwrap_or_default();

                // Weight by completion percentage
                let weight = rating;
                total_weight += weight;

                *genre_weights.entry(genre).or_insert(0.0) += weight;
                *director_weights.entry(director).or_insert(0.0) += weight;
            }

            // Normalize weights
            if total_weight > 0.0 {
                for w in genre_weights.values_mut() {
                    *w /= total_weight;
                }
                for w in director_weights.values_mut() {
                    *w /= total_weight;
                }
            }

            // Create embedding vector (normalized watched movie IDs)
            let embedding_vector: Vec<f64> = watches
                .iter()
                .take(100) // Limit to 100 dimensions
                .map(|(_, completion)| completion / 100.0)
                .collect();

            // Store embedding  
            sqlx::query(  
                r#"INSERT INTO user_embeddings (profile_id, embedding_vector, genre_weights, director_weights, calculated_at, version)  
                   VALUES ($1, $2, $3, $4, NOW(), 1)  
                   ON CONFLICT(profile_id) DO UPDATE SET  
                   embedding_vector = $2,  
                   genre_weights = $3,  
                   director_weights = $4,  
                   calculated_at = NOW(),  
                   version = version + 1"#,  
            )  
            .bind(profile_id)  
            .bind(embedding_vector)  
            .bind(serde_json::to_string(&genre_weights).unwrap())  
            .bind(serde_json::to_string(&director_weights).unwrap())  
            .execute(pool)  
            .await?;  

            count += 1;  
            if count % 100 == 0 {  
                println!("  Processed {} user embeddings...", count);  
            }  
        }

        Ok(count)
    }

        /// Job 2: Pre-calculate Movie Similarities (Daily, 2:30 AM UTC)  
    pub async fn precalculate_movie_similarities(  
        pool: Arc<PgPool>,  
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {  
        let executor = JobExecutor::new(pool.clone());  
        let job_name = "precalculate_movie_similarities";  
        let job_id = executor.start_job(job_name).await?;  

        match Self::_precalculate_similarities_impl(pool.as_ref()).await {  
            Ok(count) => {  
                executor.complete_job(job_id, job_name).await?;  
                println!("🎬 Pre-calculated {} movie similarities", count);  
                Ok(())  
            }  
            Err(e) => {  
                executor.fail_job(job_id, job_name, &e.to_string()).await?;  
                Err(e)  
            }  
        }  
    }  

     async fn _precalculate_similarities_impl(pool: &PgPool) -> Result<i32,  Box<dyn std::error::Error + Send + Sync>> {  
        // Get top 500 movies by watch count  
        let movies: Vec<Uuid> = sqlx::query_scalar(  
            r#"SELECT m.id FROM movies m  
               JOIN watch_history wh ON m.id = wh.movie_id  
               GROUP BY m.id  
               ORDER BY COUNT(*) DESC  
               LIMIT 500"#,  
        )  
        .fetch_all(pool)  
        .await?;  

        let mut count = 0;  

        for movie_id in movies {  
            // Get candidates  
            let candidates: Vec<Uuid> = sqlx::query_scalar(  
                r#"SELECT id FROM movies WHERE id != $1 AND status = 'completed' LIMIT 100"#,  
            )  
            .bind(movie_id)  
            .fetch_all(pool)  
            .await?;  

            for candidate_id in candidates {  
                // Calculate similarity  
                let similarity = Self::calculate_movie_similarity(pool, movie_id, candidate_id)  
                    .await  
                    .unwrap_or(0.0);  

                if similarity > 0.15 {  
                    sqlx::query(  
                        r#"INSERT INTO movie_similarity_scores (movie_a_id, movie_b_id, similarity_score, similarity_type, calculated_at)  
                           VALUES ($1, $2, $3, 'content', NOW())  
                           ON CONFLICT(movie_a_id, movie_b_id, similarity_type) DO UPDATE SET  
                           similarity_score = $3,  
                           calculated_at = NOW()"#,  
                    )  
                    .bind(movie_id)  
                    .bind(candidate_id)  
                    .bind(similarity)  
                    .execute(pool)  
                    .await?;  

                    count += 1;  
                }  
            }  
        }  

        Ok(count)  
    }  

    async fn calculate_movie_similarity(  
        pool: &PgPool,  
        movie_a: Uuid,  
        movie_b: Uuid,  
    ) -> Result<f64, sqlx::Error> {  
        let (a_genre, a_director, a_year): (String, String, i32) =  
            sqlx::query_as("SELECT genre, director, release_year FROM movies WHERE id = $1")  
                .bind(movie_a)  
                .fetch_one(pool)  
                .await?;  

        let (b_genre, b_director, b_year): (String, String, i32) =  
            sqlx::query_as("SELECT genre, director, release_year FROM movies WHERE id = $1")  
                .bind(movie_b)  
                .fetch_one(pool)  
                .await?;  

        let mut score: f64 = 0.0;  

        // Genre match (40%)  
        if a_genre == b_genre {  
            score += 0.4;  
        }  

        // Director match (30%)  
        if a_director == b_director {  
            score += 0.3;  
        }  

        // Release year proximity (20%)  
        if (a_year - b_year).abs() <= 5 {  
            score += 0.2;  
        }  

        // Co-watch pattern (10%)  
        let co_watch: i64 = sqlx::query_scalar(  
            r#"SELECT COUNT(*) FROM watch_history wh1  
               JOIN watch_history wh2 ON wh1.profile_id = wh2.profile_id  
               WHERE wh1.movie_id = $1 AND wh2.movie_id = $2"#,  
        )  
        .bind(movie_a)  
        .bind(movie_b)  
        .fetch_one(pool)  
        .await?;  

        if co_watch > 5 {  
            score += 0.1;  
        }  

        Ok(score.min(1.0))  
    }  

    /// Job 3: Warm Recommendation Cache - Daily, 3 AM UTC
    pub async fn warm_recommendation_cache(
        pool: Arc<PgPool>,
        redis: Arc<RedisPool>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let executor = JobExecutor::new(pool.clone());
        let job_name = "warm_recommendation_cache";
        let job_id = executor.start_job(job_name).await?;

        match Self::_warm_cache_impl(pool.clone(), redis.clone()).await {
            Ok(_) => {
                executor.complete_job(job_id, job_name).await?;
                println!("✅ Cache warming completed successfully");
                Ok(())
            }
            Err(e) => {
                executor.fail_job(job_id, job_name, &e.to_string()).await?;
                Err(e)
            }
        }
    }

    async fn _warm_cache_impl(  
        pool: Arc<PgPool>,  
        redis: Arc<RedisPool>,  
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {  
        let mut warmer = CacheWarmer::new(redis.get_connection(), pool.clone());  
        warmer.warm_active_users().await?;  
        Ok(())  
    }  

    /// Job 4: Recalculate User Segments - Weekly, Sunday 3 AM UTC
    pub async fn recalculate_user_segments(
        pool: Arc<PgPool>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let executor = JobExecutor::new(pool.clone());
        let job_name = "recalculate_user_segments";
        let job_id = executor.start_job(job_name).await?;

        match Self::_recalculate_segments_impl(pool.as_ref()).await {
            Ok(count) => {
                executor.complete_job(job_id, job_name).await?;
                println!("📊 Recalculated segments for {} users", count);
                Ok(())
            }
            Err(e) => {
                executor.fail_job(job_id, job_name, &e.to_string()).await?;
                Err(e)
            }
        }
    }

      async fn _recalculate_segments_impl(pool: &PgPool) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {  
        let users: Vec<Uuid> = sqlx::query_scalar(  
            "SELECT DISTINCT profile_id FROM watch_history WHERE watched_at > NOW() - INTERVAL '90 days'"  
        )  
        .fetch_all(pool)  
        .await?;  

        let mut count = 0;  

        for profile_id in users {  
            // Get watch metrics  
            let (total_watches, avg_completion, favorite_genre): (i64, f64, Option<String>) =  
                sqlx::query_as(  
                    r#"SELECT COUNT(*), AVG(completion_percentage),   
                             (SELECT m.genre FROM watch_history wh  
                              JOIN movies m ON wh.movie_id = m.id  
                              WHERE wh.profile_id = $1 AND wh.completed = true  
                              GROUP BY m.genre  
                              ORDER BY COUNT(*) DESC LIMIT 1)  
                       FROM watch_history WHERE profile_id = $1"#,  
                )  
                .bind(profile_id)  
                .fetch_one(pool)  
                .await?;  

            // Determine segment  
            let segment = if total_watches > 50 && avg_completion > 80.0 {  
                "power_user"  
            } else if total_watches > 20 && avg_completion > 60.0 {  
                "active_user"  
            } else if total_watches > 5 {  
                "casual_user"  
            } else {  
                "new_user"  
            };  

            // Update user segment  
            sqlx::query(  
                r#"INSERT INTO user_segments (profile_id, segment_type, primary_genre, updated_at)  
                   VALUES ($1, $2, $3, NOW())  
                   ON CONFLICT(profile_id) DO UPDATE SET  
                   segment_type = $2,  
                   primary_genre = $3,  
                   updated_at = NOW()"#,  
            )  
            .bind(profile_id)  
            .bind(segment)  
            .bind(favorite_genre)  
            .execute(pool)  
            .await?;  

            count += 1;  
        }  

        Ok(count)  
    }

    /// Job 5: Data Quality Checks (Daily, 4 AM UTC)  
    pub async fn run_data_quality_checks(  
        pool: Arc<PgPool>,  
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {  
        let executor = JobExecutor::new(pool.clone());  
        let job_name = "data_quality_checks";  
        let job_id = executor.start_job(job_name).await?;  

        match Self::_data_quality_checks_impl(pool.as_ref()).await {  
            Ok(issues) => {  
                executor.complete_job(job_id, job_name).await?;  
                println!("🔍 Found {} data quality issues", issues);  
                Ok(())  
            }  
            Err(e) => {  
                executor.fail_job(job_id, job_name, &e.to_string()).await?;  
                Err(e)  
            }  
        }  
    }  

    async fn _data_quality_checks_impl(pool: &PgPool) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {  
        let mut issues = 0;  

        // Check 1: Duplicate watch entries  
        let duplicates: i64 = sqlx::query_scalar(  
            r#"SELECT COUNT(*) FROM (  
               SELECT profile_id, movie_id, COUNT(*) as cnt  
               FROM watch_history  
               GROUP BY profile_id, movie_id  
               HAVING COUNT(*) > 1  
            ) t"#,  
        )  
        .fetch_one(pool)  
        .await?;  
        issues += duplicates as i32;  

        if duplicates > 0 {  
            println!("  ⚠️ Found {} duplicate watch entries", duplicates);  
        }  

        // Check 2: Invalid completion percentages  
        let invalid_completion: i64 = sqlx::query_scalar(  
            "SELECT COUNT(*) FROM watch_history WHERE completion_percentage < 0 OR completion_percentage > 100"  
        )  
        .fetch_one(pool)  
        .await?;  
        issues += invalid_completion as i32;  

        if invalid_completion > 0 {  
            println!("  ⚠️ Found {} invalid completion percentages", invalid_completion);  
        }  

        // Check 3: Orphaned watch records (no movie)  
        let orphaned: i64 = sqlx::query_scalar(  
            r#"SELECT COUNT(*) FROM watch_history wh  
               WHERE NOT EXISTS (SELECT 1 FROM movies m WHERE m.id = wh.movie_id)"#,  
        )  
        .fetch_one(pool)  
        .await?;  
        issues += orphaned as i32;  

        if orphaned > 0 {  
            println!("  ⚠️ Found {} orphaned watch records", orphaned);  
            // Clean up  
            sqlx::query(  
                r#"DELETE FROM watch_history wh  
                   WHERE NOT EXISTS (SELECT 1 FROM movies m WHERE m.id = wh.movie_id)"#,  
            )  
            .execute(pool)  
            .await?;  
        }  

        Ok(issues)  
    }    
}