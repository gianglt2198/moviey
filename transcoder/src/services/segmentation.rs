// use serde_json::json;
// use sqlx::{Error, PgPool};
// use uuid::Uuid;

// pub use crate::models::*;

// pub struct SegmentationService;

// impl SegmentationService {
//     pub async fn calculate_and_store_segment(pool: &PgPool) -> Result<(), Error> {
//         let profiles = sqlx::query_as::<_, (Uuid,)>(
//             "SELECT id FROM profiles WHERE created_at > NOW() - INTERVAL '1 days'",
//         )
//         .fetch_all(pool)
//         .await?;

//         for (profile_id,) in profiles {
//             let segment = Self::calculate_user_segment(pool, profile_id).await?;
//             Self::store_user_segment(pool, profile_id, segment).await?;
//         }

//         Ok(())
//     }

//     async fn calculate_user_segment(
//         pool: &PgPool,
//         profile_id: Uuid,
//     ) -> Result<UserSegmentResponse, Error> {
//         let stats = sqlx::query_as::<_, (i64, f64, i64, f64)>(
//             r#"SELECT
//                    COUNT(DISTINCT wh.movie_id) as total_movies_watched,
//                    AVG(wh.completion_percentage) as avg_completion,
//                    COUNT(DISTINCT m.genre) as genre_diversity,
//                    COUNT(*) FILTER (
//                        WHERE EXTRACT(DOW FROM wh.watched_at) IN (5,6,0)
//                    )::float / NULLIF(COUNT(*), 0) as weekend_ratio
//                 FROM watch_movies wh JOIN movies m ON wh.movie_id = m.id
//                 WHERE wh.profile_id = $1
//                 AND wh.watched_at > NOW() - INTERVAL '7 days'
//             "#,
//         )
//         .bind(profile_id)
//         .fetch_one(pool)
//         .await?;

//         let (movies_week, avg_completion, genre_diversity, weekend_ratio) = stats;

//         let (segment_type, score, characteristics) = if movies_week >= 5 && avg_completion >= 85.0 {
//             (
//                 "BINGE_WATCHER",
//                 0.95,
//                 vec![
//                     "High volume watcher".to_string(),
//                     "Completes most movies".to_string(),
//                     "Dedicated viewer".to_string(),
//                 ],
//             )
//         } else if movies_week >= 1 && movies_week <= 4 && avg_completion >= 70.0 {
//             (
//                 "CASUAL_VIEWER",
//                 0.85,
//                 vec![
//                     "Watches occasionally".to_string(),
//                     "Completes most movies".to_string(),
//                     "Enjoys variety".to_string(),
//                 ],
//             )
//         } else if genre_diversity >= 5 && weekend_ratio >= 0.5 {
//             (
//                 "EXPLORER",
//                 0.90,
//                 vec![
//                     "Loves discovering new genres".to_string(),
//                     "Watches mostly on weekends".to_string(),
//                     "Adventurous viewer".to_string(),
//                 ],
//             )
//         } else if weekend_ratio >= 0.8 {
//             (
//                 "WEEKEND_WARRIOR",
//                 0.75,
//                 vec![
//                     "Weekend binge watcher".to_string(),
//                     "Predictable patterns".to_string(),
//                     "Works during week".to_string(),
//                 ],
//             )
//         } else if movies_week == 0 {
//             (
//                 "INACTIVE",
//                 0.50,
//                 vec![
//                     "No recent activity".to_string(),
//                     "Churn risk".to_string(),
//                     "Needs re-engagement".to_string(),
//                 ],
//             )
//         } else {
//             ("GENERAL", 0.60, vec!["Standard user".to_string()])
//         };
//         let recommendations = vec![
//             "Personalized recommendations".to_string(),
//             "Trending content".to_string(),
//             "Similar movies".to_string(),
//         ];

//         Ok(UserSegmentResponse {
//             segment_type: segment_type.to_string(),
//             segment_score: score,
//             description: format!("User is a {}", segment_type),
//             characteristics,
//             recommendations,
//         })
//     }

//     async fn store_user_segment(
//         pool: &PgPool,
//         profile_id: Uuid,
//         segment: UserSegmentResponse,
//     ) -> Result<(), Error> {
//        sqlx::query(
//             r#"INSERT INTO user_segments
//                (profile_id, segment_type, segment_score, last_calculated_at, valid_until, metadata)
//             VALUES ($1, $2, $3, NOW(), NOW() + INTERVAL '7 days', $4)
//             ON CONFLICT (profile_id) DO UPDATE
//             SET segment_type = EXCLUDED.segment_type,
//                 segment_score = EXCLUDED.segment_score,
//                 last_calculated_at = NOW(),
//                 valid_until = NOW() + INTERVAL '7 days',
//                 metadata = EXCLUDED.metadata"#
//         )
//         .bind(profile_id)
//         .bind(&segment.segment_type)
//         .bind(segment.segment_score)
//         .bind(json!({
//             "description": segment.description,
//             "characteristics": segment.characteristics,
//         }))
//         .execute(pool)
//         .await?;

//         Ok(())
//     }
// }
