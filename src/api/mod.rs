pub mod error;
pub mod handlers;
pub mod routes;

// Re-export main router
pub use routes::router;
// Re-export errors
pub use error::{ApiError, ApiResult};
