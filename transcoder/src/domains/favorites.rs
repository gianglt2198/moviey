use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

/// Domain model - represents the favorites table
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Favorite {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub movie_id: Uuid,
    pub created_at: DateTime<Utc>,
}
