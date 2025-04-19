pub mod error;
pub mod handlers;
pub mod routes;

// Re-export main router
pub use routes::router;
// Re-export errors
// Commented out as they're flagged as unused, but they may be used indirectly.
// Reinstate if needed for external crates or future development
// pub use error::{ApiError, ApiResult};
