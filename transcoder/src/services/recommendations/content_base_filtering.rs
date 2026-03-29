use sqlx::PgPool;
use uuid::Uuid;

pub struct ContentBaseFiltering;

impl ContentBaseFiltering {
    /// Extract user's content preferences from watch history  
    pub async fn extract_user_preferences(
        pool: &PgPool,
        profile_id: Uuid,
    ) -> Result<(Vec<(String, f64)>, Vec<(String, f64)>), sqlx::Error> {
        // Get genre preferences weighted by completion rate
        let genre_prefs = sqlx::query_as::<_, (String, f64)>(
            r#"SELECT m.genre, AVG(wh.completion_percentage) as avg_completion  
               FROM watch_history wh  
               JOIN movies m ON wh.movie_id = m.id  
               WHERE wh.profile_id = $1 AND wh.completed = true  
               GROUP BY m.genre  
               ORDER BY avg_completion DESC  
               LIMIT 10"#,
        )
        .bind(profile_id)
        .fetch_all(pool)
        .await?;

        // Get director preferences
        let director_prefs = sqlx::query_as::<_, (String, f64)>(
            r#"SELECT m.director, COUNT(*) as watch_count  
               FROM watch_history wh  
               JOIN movies m ON wh.movie_id = m.id  
               WHERE wh.profile_id = $1 AND wh.completed = true  
               GROUP BY m.director  
               ORDER BY watch_count DESC  
               LIMIT 10"#,
        )
        .bind(profile_id)
        .fetch_all(pool)
        .await?;

        Ok((genre_prefs, director_prefs))
    }

    /// Calculate content-based similarity score  
    pub async fn calculate_content_score(
        pool: &PgPool,
        profile_id: Uuid,
        movie_id: Uuid,
    ) -> Result<f64, sqlx::Error> {
        let (genre_prefs, director_prefs) =
            Self::extract_user_preferences(pool, profile_id).await?;

        let movie = sqlx::query_as::<_, (String, String, i32, f64)>(
            "SELECT genre, director, release_year, rating FROM movies WHERE id = $1",
        )
        .bind(movie_id)
        .fetch_optional(pool)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

        let mut score = 0.0;

        // Genre match (60% weight)
        if let Some((_, pref_score)) = genre_prefs.iter().find(|(g, _)| g == &movie.0) {
            score += 0.6 * (*pref_score / 100.0);
        }

        // Director match (25% weight)
        if let Some((_, pref_score)) = director_prefs.iter().find(|(d, _)| d == &movie.1) {
            score += 0.25 * (*pref_score / 10.0).min(1.0);
        }

        // Rating boost (15% weight)
        score += 0.15 * (movie.3 / 10.0).min(1.0);

        Ok(score.min(1.0))
    }
}
