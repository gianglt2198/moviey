use axum::Json;
use axum::http::StatusCode;
use axum::{
    extract::{State},
    response::Result,
};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use sqlx::{PgPool, Row};

use crate::models::*;

pub async fn toggle_favorite(  
    claims: Claims,  
    State(pool): State<PgPool>,  
    Json(payload): Json<FavoriteRequest>,  
) -> Result<(), StatusCode> {  
    let profile_id: Option<uuid::Uuid> = sqlx::query_scalar(  
        "SELECT id FROM profiles WHERE user_id = $1 LIMIT 1"  
    )  
    .bind(claims.sub)  
    .fetch_optional(&pool)  
    .await  
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;  

    let profile_id = match profile_id {  
        Some(id) => id,  
        None => {  
            sqlx::query_scalar::<_, uuid::Uuid>(  
                "INSERT INTO profiles (user_id, name) VALUES ($1, $2) RETURNING id"  
            )  
            .bind(claims.sub)  
            .bind("Default Profile")  
            .fetch_one(&pool)  
            .await  
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?  
        }  
    };  

    // Check if already favorited  
    let exists: bool = sqlx::query_scalar(  
        "SELECT EXISTS(SELECT 1 FROM favorites WHERE profile_id = $1 AND movie_id = $2)"  
    )  
    .bind(profile_id)  
    .bind(payload.movie_id)  
    .fetch_one(&pool)  
    .await  
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;  

    if exists {  
        // Remove favorite  
        sqlx::query!(  
            "DELETE FROM favorites WHERE profile_id = $1 AND movie_id = $2",  
            profile_id,  
            payload.movie_id  
        )  
        .execute(&pool)  
        .await  
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;  
    } else {  
        // Add favorite  
        sqlx::query!(  
            "INSERT INTO favorites (profile_id, movie_id) VALUES ($1, $2)",  
            profile_id,  
            payload.movie_id  
        )  
        .execute(&pool)  
        .await  
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;  
    }  

    Ok(())  
}  

pub async fn get_favorites(  
    claims: crate::models::Claims,  
    State(pool): State<PgPool>,  
) -> Result<Json<Vec<MovieDetailDto>>, StatusCode> {  
    let profile_id: Option<uuid::Uuid> = sqlx::query_scalar(  
        "SELECT id FROM profiles WHERE user_id = $1 LIMIT 1"  
    )  
    .bind(claims.sub)  
    .fetch_optional(&pool)  
    .await  
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;  

    let profile_id = profile_id.ok_or(StatusCode::NOT_FOUND)?;  

    let rows = sqlx::query(  
        r#"SELECT m.id, m.title, m.status, m.duration_seconds, m.genre, m.director, m.release_year, m.rating, m.description  
           FROM movies m  
           JOIN favorites f ON m.id = f.movie_id  
           WHERE f.profile_id = $1 AND m.status = 'completed'  
           ORDER BY f.created_at DESC"#  
    )  
    .bind(profile_id)  
    .fetch_all(&pool)  
    .await  
    .unwrap_or_default();  

    let movies = rows  
        .into_iter()  
        .map(|row| {  
            let title: String = row.get(1);  
            let rating = row.get::<Option<Decimal>, _>(7).map(|r| r.to_f64().unwrap_or(0.0));
            MovieDetailDto {  
                id: row.get(0),  
                title: title.clone(),  
                stream_url: format!(  
                    "http://localhost:3000/streams/{}/master.m3u8",  
                    title  
                ),  
                status: row.get(2),  
                duration: format!("{} min", row.get::<i32, _>(3) / 60),  
                thumbnail_url: format!(  
                    "http://localhost:3000/streams/{}/thumbnail.jpg",  
                    title  
                ),  
                genre: row.get(4),  
                director: row.get(5),  
                release_year: row.get(6),  
                rating: rating,  
                description: row.get(8),  
            }  
        })  
        .collect();  

    Ok(Json(movies))  
}  