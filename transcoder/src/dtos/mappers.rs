use rust_decimal::prelude::ToPrimitive;

use crate::domains::*;
use crate::dtos::*;

pub fn map_movie_to_response(movie: Movie) -> MovieResponse {
    MovieResponse {
        id: movie.id,
        title: movie.title.clone(),
        stream_url: format!("http://localhost:3000/streams/{}/master.m3u8", movie.title),
        status: format!("{:?}", movie.status),
        duration: format!("{} min", movie.duration_seconds.unwrap_or(0) / 60),
        thumbnail_url: format!(
            "http://localhost:3000/streams/{}/thumbnail.jpg",
            movie.title
        ),
    }
}

pub fn map_movie_to_detail_response(movie: Movie) -> MovieDetailResponse {
    MovieDetailResponse {
        id: movie.id,
        title: movie.title.clone(),
        stream_url: format!("http://localhost:3000/streams/{}/master.m3u8", movie.title),
        status: format!("{:?}", movie.status),
        duration: format!("{} min", movie.duration_seconds.unwrap_or(0) / 60),
        thumbnail_url: format!(
            "http://localhost:3000/streams/{}/thumbnail.jpg",
            movie.title
        ),
        genre: movie.genre,
        director: movie.director,
        release_year: movie.release_year,
        rating: movie.rating.map(|r| r.to_f64().unwrap_or(0.0)),
        description: movie.description,
    }
}

pub fn map_user_to_profile_response(user: User, profile: Option<Profile>) -> UserProfileResponse {
    UserProfileResponse {
        id: user.id,
        email: user.email,
        created_at: user.created_at.unwrap_or(chrono::Utc::now()),
        profile: profile.map(map_profile_to_response),
    }
}

pub fn map_profile_to_response(profile: Profile) -> ProfileResponse {
    ProfileResponse {
        id: profile.id,
        name: profile.name,
        avatar_url: profile.avatar_url,
        created_at: profile.created_at,
    }
}

// Map WatchHistory domain to WatchHistoryResponse DTO
pub fn map_watch_history_to_response(history: WatchHistory) -> WatchHistoryResponse {
    WatchHistoryResponse {
        id: history.id,
        movie_id: history.movie_id,
        last_position_seconds: history.last_position_seconds,
        completed: history.completed,
        watched_at: history.watched_at.to_rfc3339(),
    }
}

// Map WatchHistory domain to WatchHistoryDetailResponse DTO
pub fn map_watch_history_to_detail_response(history: WatchHistory) -> WatchHistoryDetailResponse {
    WatchHistoryDetailResponse {
        id: history.id,
        movie_id: history.movie_id,

        last_position_seconds: history.last_position_seconds,
        completed: history.completed,
        watch_duration_seconds: history.watch_duration_seconds,
        total_movie_duration_seconds: history.total_movie_duration_seconds,
        completion_percentage: history
            .completion_percentage
            .map(|p| p.to_f64().unwrap_or(0.0)),
        watch_quality: history.watch_quality,
        interrupted_count: history.interrupted_count,
        playback_speed: history.playback_speed.map(|p| p.to_f64().unwrap_or(1.0)),
        device_type: history.device_type,
        completion_reason: history.completion_reason,
        flagged_for_review: history.flagged_for_review,
        watched_at: history.watched_at.to_rfc3339(),
    }
}
