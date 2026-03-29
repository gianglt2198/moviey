use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    pub sub: uuid::Uuid, // User ID
    pub exp: i64,        // Expiration timestamp
}
