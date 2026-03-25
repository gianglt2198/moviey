use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, sqlx::Type)]
#[sqlx(type_name = "movie_status", rename_all = "lowercase")]
pub enum MovieStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieDto {
    pub id: Uuid,
    pub title: String,
    pub stream_url: String, // This will point to our static file server
    pub status: MovieStatus,
    pub duration: String,
    pub thumbnail_url: String,
}
