use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    pub sub: uuid::Uuid, // User ID
    pub exp: i64,        // Expiration timestamp
}

#[derive(Serialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserProfile {
    pub id: uuid::Uuid,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
