// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

use axum::{
    Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post},
};
use ontology_core::db::repository::{RepositoryError, RepositoryFactory};
use ontology_core::domain::Symbol;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct SymbolQuery {
    category: Option<String>,
    query: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    status: String,
    data: T,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    status: String,
    message: String,
}

pub fn router<F: RepositoryFactory + Send + Sync + 'static>(repo_factory: Arc<F>) -> Router {
    Router::new()
        .route("/symbols", get(list_symbols::<F>))
        .route("/symbols/:id", get(get_symbol::<F>))
        .route("/symbols", post(create_symbol::<F>))
        .route("/symbols/:id", delete(delete_symbol::<F>))
        .with_state(repo_factory)
}

async fn list_symbols<F: RepositoryFactory + Send + Sync + 'static>(
    State(repo_factory): State<Arc<F>>,
    Query(params): Query<SymbolQuery>,
) -> Result<Json<ApiResponse<Vec<Symbol>>>, (StatusCode, Json<ErrorResponse>)> {
    let repo = repo_factory.create_symbol_repository();

    let symbols = if let Some(query) = params.query {
        repo.search_symbols(&query).await
    } else {
        repo.list_symbols(params.category.as_deref()).await
    };

    match symbols {
        Ok(symbols) => Ok(Json(ApiResponse {
            status: "success".into(),
            data: symbols,
        })),
        Err(e) => Err(handle_error(e)),
    }
}

async fn get_symbol<F: RepositoryFactory + Send + Sync + 'static>(
    State(repo_factory): State<Arc<F>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<Symbol>>, (StatusCode, Json<ErrorResponse>)> {
    let repo = repo_factory.create_symbol_repository();

    match repo.get_symbol(&id).await {
        Ok(symbol) => Ok(Json(ApiResponse {
            status: "success".into(),
            data: symbol,
        })),
        Err(e) => Err(handle_error(e)),
    }
}

async fn create_symbol<F: RepositoryFactory + Send + Sync + 'static>(
    State(repo_factory): State<Arc<F>>,
    Json(symbol): Json<Symbol>,
) -> Result<Json<ApiResponse<Symbol>>, (StatusCode, Json<ErrorResponse>)> {
    let repo = repo_factory.create_symbol_repository();

    match repo.create_symbol(symbol).await {
        Ok(created) => Ok(Json(ApiResponse {
            status: "success".into(),
            data: created,
        })),
        Err(e) => Err(handle_error(e)),
    }
}

async fn delete_symbol<F: RepositoryFactory + Send + Sync + 'static>(
    State(repo_factory): State<Arc<F>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<String>>, (StatusCode, Json<ErrorResponse>)> {
    let repo = repo_factory.create_symbol_repository();

    match repo.delete_symbol(&id).await {
        Ok(()) => Ok(Json(ApiResponse {
            status: "success".into(),
            data: format!("Symbol {} deleted", id),
        })),
        Err(e) => Err(handle_error(e)),
    }
}

fn handle_error(err: RepositoryError) -> (StatusCode, Json<ErrorResponse>) {
    let (status, message) = match &err {
        RepositoryError::NotFound(_) => (StatusCode::NOT_FOUND, err.to_string()),
        RepositoryError::Conflict(_) => (StatusCode::CONFLICT, err.to_string()),
        RepositoryError::Validation(_) => (StatusCode::BAD_REQUEST, err.to_string()),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        ),
    };

    (
        status,
        Json(ErrorResponse {
            status: "error".into(),
            message,
        }),
    )
}
