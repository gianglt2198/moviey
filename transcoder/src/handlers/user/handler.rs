use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::PgPool;

use crate::{
    domains::{Profile, User},
    dtos::{map_profile_to_response, user_dto::*},
    handlers::user::validators::validators,
    models::Claims,
};

const SECRET_KEY: &[u8] = b"secret_key";

/// Register a new user  
#[utoipa::path(  
    post,  
    path = "/api/auth/register",  
    request_body = RegisterRequest,  
    responses(  
        (status = 201, description = "User registered successfully"),  
        (status = 400, description = "Invalid email or password format"),  
        (status = 409, description = "Email already registered")  
    ),  
    tag = "Authentication"  
)]  
pub async fn register(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    validators::validate_email(&payload.email).map_err(|_| StatusCode::BAD_REQUEST)?;
    validators::validate_password(&payload.password).map_err(|_| StatusCode::BAD_REQUEST)?;

    let hashed =
        hash(&payload.password, DEFAULT_COST).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query!(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2)",
        payload.email,
        hashed
    )
    .execute(pool.as_ref())
    .await
    .map_err(|_| StatusCode::CONFLICT)?;

    Ok(StatusCode::CREATED)
}

/// Login user and return JWT token  
#[utoipa::path(  
    post,  
    path = "/api/auth/login",  
    request_body = LoginRequest,  
    responses(  
        (status = 200, description = "Login successful", body = AuthResponse),  
        (status = 401, description = "Invalid credentials"),  
        (status = 500, description = "Internal server error")  
    ),  
    tag = "Authentication"  
)]  
pub async fn login(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let user: User =
        sqlx::query_as::<_, User>("SELECT id, password_hash FROM users WHERE email = $1")
            .bind(payload.email)
            .fetch_one(pool.as_ref())
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let is_valid = verify(&payload.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !is_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

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

/// Get current user profile  
#[utoipa::path(  
    get,  
    path = "/api/auth/profile",  
    security(  
        ("bearer_auth" = [])  
    ),  
    responses(  
        (status = 200, description = "Profile retrieved", body = ProfileResponse),  
        (status = 401, description = "Unauthorized - invalid or missing token"),  
        (status = 404, description = "User profile not found")  
    ),  
    tag = "User"  
)]  
pub async fn get_user_profile(
    claims: Claims,
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<ProfileResponse>, StatusCode> {
    let user = sqlx::query_as::<_, Profile>(
        r#"
        SELECT 
           *
        FROM profiles p
        WHERE p.user_id = $1"#,
    )
    .bind(claims.sub)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(map_profile_to_response(user)))
}

/// Create a profile for the user  
#[utoipa::path(  
    post,  
    path = "/api/auth/profile/create",  
    request_body = CreateProfileRequest,  
    security(  
        ("bearer_auth" = [])  
    ),  
    responses(  
        (status = 200, description = "Profile created", body = ProfileResponse),  
        (status = 401, description = "Unauthorized"),  
        (status = 500, description = "Internal server error")  
    ),  
    tag = "User"  
)]  
pub async fn create_profile(
    claims: Claims,
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateProfileRequest>,
) -> Result<Json<ProfileResponse>, StatusCode> {
    let profile_id = sqlx::query_scalar::<_, uuid::Uuid>(
        "INSERT INTO profiles (user_id, name, avatar_url) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(claims.sub)
    .bind(&payload.name)
    .bind(&payload.avatar_url)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ProfileResponse {
        id: profile_id,
        name: payload.name,
        avatar_url: payload.avatar_url,
        created_at: Utc::now(),
    }))
}
