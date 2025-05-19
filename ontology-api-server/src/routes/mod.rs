// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

pub mod symbols;

use axum::Router;
use ontology_core::db::repository::RepositoryFactory;
use std::sync::Arc;

pub fn create_api_router<F: RepositoryFactory + Send + Sync + 'static>(
    repo_factory: Arc<F>,
) -> Router {
    Router::new().merge(symbols::router(repo_factory.clone()))
    // Add more routes as needed
}
