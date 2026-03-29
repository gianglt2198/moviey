use std::collections::HashSet;

use sqlx::{PgPool, pool};
use utoipa::openapi::content;
use uuid::Uuid;

use crate::services::recommendations::{
    collaborative_filtering::CollaborativeFiltering, content_base_filtering::ContentBaseFiltering,
};

pub struct HybridRecommender {
    pub collab_weight: f64,
    pub content_weight: f64,
}

impl Default for HybridRecommender {
    fn default() -> Self {
        Self {
            collab_weight: 0.4,
            content_weight: 0.6,
        }
    }
}

impl HybridRecommender {
    pub fn new(collab_weight: f64, content_weight: f64) -> Self {
        Self {
            collab_weight,
            content_weight,
        }
    }

    /// Generate personalized recommendations
    pub async fn generate_recommendation(
        &self,
        pool: &PgPool,
        profile_id: Uuid,
        limit: i32,
        diversity_factor: f64,
    ) -> Result<Vec<(Uuid, f64, String)>, sqlx::Error> {
        let watched: Vec<Uuid> =
            sqlx::query_scalar("SELECT movie_id FROM watch_history WHERE profile_id = $1")
                .bind(profile_id)
                .fetch_all(pool)
                .await?;

        let candidates: Vec<Uuid> = sqlx::query_scalar("SELECT id FROM movies WHERE id != ALL($1) and status = 'completed' ORDER BY rating DESC LIMIT 500")
            .bind(&watched)
            .fetch_all(pool)
            .await?;

        let mut scored_movies = Vec::new();

        for movie_id in candidates {
            let collab_score = self
                .calculate_collab_score(pool, profile_id, movie_id)
                .await?;

            let content_score =
                ContentBaseFiltering::calculate_content_score(pool, profile_id, movie_id).await?;

            let hybrid_score =
                (self.collab_weight * collab_score) + (self.content_weight * content_score);

            let reason = self.generate_reason(collab_score, content_score);
            scored_movies.push((movie_id, hybrid_score, reason));
        }
        // Sort by score
        scored_movies.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Apply diversity penalty
        let diverse = self
            .apply_diversity(pool, &scored_movies, diversity_factor)
            .await?;

        Ok(diverse.into_iter().take(limit as usize).collect())
    }

    async fn calculate_collab_score(
        &self,
        pool: &PgPool,
        profile_id: Uuid,
        movie_id: Uuid,
    ) -> Result<f64, sqlx::Error> {
        let similar_users =
            CollaborativeFiltering::calculate_user_similarity(pool, profile_id).await?;
        if similar_users.is_empty() {
            return Ok(0.0);
        }

        let mut score = 0.0;
        let mut count = 0;

        for (user_id, similarity) in similar_users {
            let watched: bool = sqlx::query_scalar("SELECT COUNT(*) > 0 FROM watch_history WHERE profile_id = $1 AND movie_id = $2 AND completed = true")
                .bind(user_id)
                .bind(movie_id)
                .fetch_one(pool)
                .await?;
            if watched {
                score += similarity;
                count += 1;
            }
        }

        Ok(if count > 0 { score / count as f64 } else { 0.0 })
    }

    fn generate_reason(&self, collab: f64, content: f64) -> String {
        if collab > content {
            "Users like you watched this".to_string()
        } else if content > 0.7 {
            "Matches your content preferences".to_string()
        } else {
            "Trending recommendation".to_string()
        }
    }

    async fn apply_diversity(
        &self,
        pool: &PgPool,
        movies: &[(Uuid, f64, String)],
        diversity_factor: f64,
    ) -> Result<Vec<(Uuid, f64, String)>, sqlx::Error> {
        if diversity_factor <= 0.0 || movies.is_empty() {
            return Ok(movies.to_vec());
        }

        let mut result = vec![];
        let mut selected_genres = HashSet::new();

        for (movie_id, score, reason) in movies {
            let genre: String = sqlx::query_scalar("SELECT genre from movies where id  =$1")
                .bind(movie_id)
                .fetch_one(pool)
                .await?;

            let should_add = if selected_genres.contains(&genre) {
                score * (1.0 - diversity_factor) > 0.3
            } else {
                true
            };

            if should_add {
                result.push((*movie_id, *score, reason.clone()));
                selected_genres.insert(genre);
            }
        }
        Ok(result)
    }
}
