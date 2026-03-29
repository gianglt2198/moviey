use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// ============ REQUEST DTOs ============

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProfileRequest {
    pub name: String,
    pub avatar_url: Option<String>,
}

// ============ RESPONSE DTOs ============

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
    pub token_type: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserProfileResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub profile: Option<ProfileResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfileResponse {
    pub id: Uuid,
    pub name: String,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
}
