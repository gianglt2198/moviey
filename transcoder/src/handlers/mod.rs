pub mod analytics;
pub mod favorites;
pub mod movie;
pub mod recommendation;
pub mod user;
pub mod watch_history;

pub use analytics::router as analytics_router;
pub use favorites::router as favorites_router;
pub use movie::router as movie_router;
pub use recommendation::router as recommendation_router;
pub use user::router as user_router;
pub use watch_history::router as watch_history_router;
