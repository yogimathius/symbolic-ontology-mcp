use axum::{
    Router,
    routing::{get, post},
};

use super::handlers;

/// Builds the main application router with all API routes
pub fn router() -> Router {
    Router::new()
        .route("/health", get(handlers::health_check))
        .route("/symbols", get(handlers::list_symbols))
        .route("/symbols/{id}", get(handlers::get_symbol))
        .route("/interpret", post(handlers::interpret_symbol))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_creation() {
        let _router = router();
        // Just testing that it builds - actual routes tested in integration tests
    }
}
