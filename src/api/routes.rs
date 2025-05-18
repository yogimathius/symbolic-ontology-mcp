use axum::{
    Router,
    routing::{delete, get, post},
};
use sqlx::PgPool;

use super::{handlers, state::AppState};

pub fn router(db_pool: PgPool) -> Router {
    let app_state = AppState::new(db_pool.clone());

    let repository_api = Router::new()
        .route("/symbols", get(handlers::repo_list_symbols))
        .route("/symbols/{id}", get(handlers::repo_get_symbol))
        .route("/symbols/{id}", post(handlers::repo_update_symbol))
        .route("/symbols", post(handlers::repo_create_symbol))
        .route("/symbols/{id}", delete(handlers::repo_delete_symbol))
        .route("/symbols/{id}/related", post(handlers::add_related_symbol))
        .route("/symbol-sets", get(handlers::list_symbol_sets))
        .route("/symbol-sets/{id}", get(handlers::get_symbol_set))
        .route("/symbol-sets/search", get(handlers::search_symbol_sets))
        .route("/symbol-sets", post(handlers::create_symbol_set))
        .route("/symbol-sets/{id}", post(handlers::update_symbol_set))
        .route("/symbol-sets/{id}", delete(handlers::delete_symbol_set))
        .with_state(app_state);

    repository_api
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_router_creation() {
        // Create a mock pool without connecting to a real database
        let pool = PgPool::connect_lazy("postgres://mock:mock@mock/mock")
            .expect("Failed to create mock pool");

        // This just tests that the router builds successfully
        let _router = router(pool);
        // If we get here, the test passed
    }
}
