use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::PgPool;

use crate::models::*;

mod validators {
    pub fn validate_email(email: &str) -> Result<(), String> {
        // Simple email validation
        if email.contains('@') && email.contains('.') && email.len() > 5 {
            Ok(())
        } else {
            Err("Invalid email format".to_string())
        }
    }
    pub fn validate_password(password: &str) -> Result<(), String> {
        if password.len() < 8 {
            return Err("Password must be at least 8 characters".to_string());
        }
        if !password.chars().any(|c| c.is_uppercase()) {
            return Err("Password must contain at least one uppercase letter".to_string());
        }
        if !password.chars().any(|c| c.is_digit(10)) {
            return Err("Password must contain at least one number".to_string());
        }
        Ok(())
    }
}

// Match this with auth_extractor.rs
const SECRET_KEY: &[u8] = b"secret_key";

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate input
    validators::validate_email(&payload.email).map_err(|_| StatusCode::BAD_REQUEST)?;
    validators::validate_password(&payload.password).map_err(|_| StatusCode::BAD_REQUEST)?;

    // 1. Hash the password
    let hashed =
        hash(payload.password, DEFAULT_COST).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 2. Save to DB
    sqlx::query!(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2)",
        payload.email,
        hashed
    )
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::CONFLICT)?; // Email already exists

    Ok(StatusCode::CREATED)
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let user = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE email = $1",
        payload.email
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let is_valid = verify(payload.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !is_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Generate JWT
    let claims = Claims {
        sub: user.id,
        exp: (Utc::now() + Duration::hours(24)).timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        token,
        token_type: "Bearer".to_string(),
    }))
}

pub async fn get_user_profile(
    claims: Claims,
    State(pool): State<PgPool>,
) -> Result<Json<UserProfile>, StatusCode> {
    let user = sqlx::query!(
        "SELECT id, email, created_at FROM users WHERE id = $1",
        claims.sub
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(UserProfile {
        id: user.id,
        email: user.email,
        created_at: user.created_at.expect("created_at should not be null"),
    }))
}
