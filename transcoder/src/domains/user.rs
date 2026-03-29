use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Domain model - represents the users table
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<DateTime<Utc>>,
}

/// Domain model - represents the profiles table
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Profile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
}
