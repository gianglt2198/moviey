use std::collections::HashMap;

use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct CollaborativeFiltering;

impl CollaborativeFiltering {
    pub async fn calculate_user_similarity(
        pool: &PgPool,
        profile_id: Uuid,
    ) -> Result<Vec<(Uuid, f64)>, Error> {
    // Get target user's watch history with ratings  
        let target_watches = sqlx::query_as::<_, (Uuid, i32)>(  
            r#"SELECT movie_id, COALESCE(CAST(ROUND(completion_percentage / 20) AS INT), 0) as rating  
               FROM watch_history   
               WHERE profile_id = $1 AND completed = true  
               ORDER BY watched_at DESC LIMIT 100"#,  
        )  
        .bind(profile_id)  
        .fetch_all(pool)  
        .await?;  


       if target_watches.is_empty() {  
            return Ok(vec![]);  
        }  

         // Get similar users (users who watched same movies)  
        let similar_users = sqlx::query_as::<_, (Uuid,)>(  
            r#"SELECT DISTINCT wh2.profile_id  
               FROM watch_history wh1  
               JOIN watch_history wh2 ON wh1.movie_id = wh2.movie_id  
               WHERE wh1.profile_id = $1   
               AND wh2.profile_id != $1  
               AND wh1.completed = true  
               GROUP BY wh2.profile_id  
               HAVING COUNT(*) >= 3  
               LIMIT 50"#,  
        )  
        .bind(profile_id)  
        .fetch_all(pool)  
        .await?;  

        let mut similarities = Vec::new();  
        for (other_user_id,) in similar_users {  
            let other_watches = sqlx::query_as::<_, (Uuid, i32)>(  
                r#"SELECT movie_id, COALESCE(CAST(ROUND(completion_percentage / 20) AS INT), 0)  
                   FROM watch_history   
                   WHERE profile_id = $1 AND completed = true  
                   LIMIT 100"#,  
            )  
            .bind(other_user_id)  
            .fetch_all(pool)  
            .await?;  

            let similarity = Self::cosine_similarity(&target_watches, &other_watches);  
            if similarity > 0.1 {  
                similarities.push((other_user_id, similarity));  
            }  
        }  

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());  
        Ok(similarities)  
    }


       /// Calculate item-item similarity based on genre and metadata  
    pub async fn calculate_item_similarity(  
        pool: &PgPool,  
        movie_id: Uuid,  
    ) -> Result<Vec<(Uuid, f64)>, sqlx::Error> {  
        let source_movie = sqlx::query_as::<_, (String, String, i32)>(  
            "SELECT genre, director, release_year FROM movies WHERE id = $1"  
        )  
        .bind(movie_id)  
        .fetch_optional(pool)  
        .await?  
        .ok_or(sqlx::Error::RowNotFound)?;  

        // Find similar movies based on metadata  
        let similar_movies = sqlx::query_as::<_, (Uuid, String, String, i32)>(  
            r#"SELECT id, genre, director, release_year   
               FROM movies   
               WHERE id != $1 AND status = 'completed'  
               LIMIT 200"#,  
        )  
        .bind(movie_id)  
        .fetch_all(pool)  
        .await?;  

        let mut similarities = Vec::new();  

        for (other_id, other_genre, other_director, other_year) in similar_movies {  
            let mut score: f64 = 0.0;  

            // Genre match (40% weight)  
            if source_movie.0 == other_genre {  
                score += 0.4;  
            }  

            // Director match (30% weight)  
            if source_movie.1 == other_director {  
                score += 0.3;  
            }  

            // Release year proximity (20% weight) - within 5 years  
            if (source_movie.2 - other_year).abs() <= 5 {  
                score += 0.2;  
            }  

            // Temporal match (10% weight) - co-watched  
            let co_watch_count: i64 = sqlx::query_scalar(  
                r#"SELECT COUNT(*) FROM watch_history wh1  
                   JOIN watch_history wh2 ON wh1.profile_id = wh2.profile_id  
                   WHERE wh1.movie_id = $1 AND wh2.movie_id = $2"#,  
            )  
            .bind(movie_id)  
            .bind(other_id)  
            .fetch_one(pool)  
            .await?;  

            if co_watch_count > 0 {  
                score += 0.1;  
            }  

            if score > 0.15 {  
                similarities.push((other_id, score.min(1.0)));  
            }  
        }  

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());  
        Ok(similarities.into_iter().take(50).collect())  
    }  

       /// Cosine similarity between two rating vectors  
    fn cosine_similarity(vec_a: &[(Uuid, i32)], vec_b: &[(Uuid, i32)]) -> f64 {  
        let mut map_a: HashMap<Uuid, i32> = HashMap::new();  
        let mut map_b: HashMap<Uuid, i32> = HashMap::new();  

        for (movie, rating) in vec_a {  
            map_a.insert(*movie, *rating);  
        }  
        for (movie, rating) in vec_b {  
            map_b.insert(*movie, *rating);  
        }  

        let mut dot_product = 0.0;  
        let mut norm_a = 0.0;  
        let mut norm_b = 0.0;  

        for (movie, rating_a) in &map_a {  
            norm_a += (*rating_a as f64).powi(2);  
            if let Some(rating_b) = map_b.get(movie) {  
                dot_product += (*rating_a as f64) * (*rating_b as f64);  
            }  
        }  

        for (_, rating_b) in &map_b {  
            norm_b += (*rating_b as f64).powi(2);  
        }  

        if norm_a == 0.0 || norm_b == 0.0 {  
            return 0.0;  
        }  

        dot_product / (norm_a.sqrt() * norm_b.sqrt())  
    }  
}
