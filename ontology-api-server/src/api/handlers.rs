use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use super::error::{ApiError, ApiResult};
use super::state::AppState;
use ontology_core::db::models::Symbol;
use ontology_core::domain::ontology::SymbolSet;
use ontology_core::domain::symbols::Symbol as DomainSymbol;

#[derive(Serialize)]
pub struct SymbolsResponse {
    pub symbols: Vec<Symbol>,
    pub total_count: usize,
}

#[derive(Deserialize, Default)]
pub struct ListSymbolsQuery {
    pub category: Option<String>,
    pub query: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    50
}

#[derive(Serialize)]
pub struct CategoriesResponse {
    pub categories: Vec<String>,
    pub total_count: usize,
}

pub async fn get_categories(State(state): State<AppState>) -> ApiResult<Json<CategoriesResponse>> {
    let symbols = state.symbol_repository.list_symbols(None).await?;

    let categories: Vec<String> = symbols
        .iter()
        .map(|s| s.category.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    Ok(Json(CategoriesResponse {
        total_count: categories.len(),
        categories,
    }))
}

#[derive(Serialize)]
pub struct DomainSymbolsResponse {
    pub symbols: Vec<DomainSymbol>,
    pub total_count: usize,
}

pub async fn repo_list_symbols(
    State(state): State<AppState>,
    Query(params): Query<ListSymbolsQuery>,
) -> ApiResult<Json<DomainSymbolsResponse>> {
    if let Some(ref query) = params.query {
        if query.trim().is_empty() {
            return Err(ApiError::BadRequest(
                "Search query cannot be empty".to_string(),
            ));
        }
    }

    if let Some(ref category) = params.category {
        if category.trim().is_empty() {
            return Err(ApiError::BadRequest("Category cannot be empty".to_string()));
        }
    }

    let symbols = match (params.category.as_deref(), params.query.as_deref()) {
        (_, Some(query)) => state.symbol_repository.search_symbols(query).await?,
        (Some(category), None) => state.symbol_repository.list_symbols(Some(category)).await?,
        (None, None) => state.symbol_repository.list_symbols(None).await?,
    };

    let total_count = symbols.len();
    let symbols = symbols.into_iter().take(params.limit).collect();

    Ok(Json(DomainSymbolsResponse {
        symbols,
        total_count,
    }))
}

pub async fn repo_get_symbol(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> ApiResult<Json<DomainSymbol>> {
    if id.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Symbol ID cannot be empty".to_string(),
        ));
    }

    let symbol = state.symbol_repository.get_symbol(&id).await?;
    Ok(Json(symbol))
}

pub async fn repo_create_symbol(
    State(state): State<AppState>,
    Json(symbol): Json<DomainSymbol>,
) -> ApiResult<Json<DomainSymbol>> {
    if symbol.id.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Symbol ID cannot be empty".to_string(),
        ));
    }
    if symbol.name.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Symbol name cannot be empty".to_string(),
        ));
    }

    let created_symbol = state.symbol_repository.create_symbol(symbol).await?;
    Ok(Json(created_symbol))
}

pub async fn repo_update_symbol(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(symbol): Json<DomainSymbol>,
) -> ApiResult<Json<DomainSymbol>> {
    if id != symbol.id {
        return Err(ApiError::BadRequest(
            "Symbol ID in path does not match ID in body".to_string(),
        ));
    }

    if symbol.name.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Symbol name cannot be empty".to_string(),
        ));
    }

    let updated_symbol = state.symbol_repository.update_symbol(symbol).await?;
    Ok(Json(updated_symbol))
}

pub async fn repo_delete_symbol(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<StatusCode, ApiError> {
    if id.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Symbol ID cannot be empty".to_string(),
        ));
    }

    state.symbol_repository.delete_symbol(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Serialize)]
pub struct SymbolSetsResponse {
    pub symbol_sets: Vec<SymbolSet>,
    pub total_count: usize,
}

#[derive(Deserialize, Default)]
pub struct ListSymbolSetsQuery {
    pub category: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

pub async fn list_symbol_sets(
    State(state): State<AppState>,
    Query(params): Query<ListSymbolSetsQuery>,
) -> ApiResult<Json<SymbolSetsResponse>> {
    if let Some(ref category) = params.category {
        if category.trim().is_empty() {
            return Err(ApiError::BadRequest("Category cannot be empty".to_string()));
        }
    }

    let symbol_sets = match params.category.as_deref() {
        Some(category) => {
            state
                .symbol_set_repository
                .list_symbol_sets(Some(category))
                .await?
        }
        None => state.symbol_set_repository.list_symbol_sets(None).await?,
    };

    let total_count = symbol_sets.len();
    let symbol_sets = symbol_sets.into_iter().take(params.limit).collect();

    Ok(Json(SymbolSetsResponse {
        symbol_sets,
        total_count,
    }))
}

pub async fn get_symbol_set(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> ApiResult<Json<SymbolSet>> {
    if id.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Symbol set ID cannot be empty".to_string(),
        ));
    }

    let symbol_set = state.symbol_set_repository.get_symbol_set(&id).await?;
    Ok(Json(symbol_set))
}

#[derive(Deserialize)]
pub struct SearchSymbolSetsQuery {
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

pub async fn search_symbol_sets(
    State(state): State<AppState>,
    Query(params): Query<SearchSymbolSetsQuery>,
) -> ApiResult<Json<SymbolSetsResponse>> {
    if params.query.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Search query cannot be empty".to_string(),
        ));
    }

    let symbol_sets = state
        .symbol_set_repository
        .search_symbol_sets(&params.query)
        .await?;

    let total_count = symbol_sets.len();
    let symbol_sets = symbol_sets.into_iter().take(params.limit).collect();

    Ok(Json(SymbolSetsResponse {
        symbol_sets,
        total_count,
    }))
}

pub async fn create_symbol_set(
    State(state): State<AppState>,
    Json(symbol_set): Json<SymbolSet>,
) -> ApiResult<Json<SymbolSet>> {
    if symbol_set.id.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Symbol set ID cannot be empty".to_string(),
        ));
    }
    if symbol_set.name.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Symbol set name cannot be empty".to_string(),
        ));
    }

    let created_symbol_set = state
        .symbol_set_repository
        .create_symbol_set(symbol_set)
        .await?;
    Ok(Json(created_symbol_set))
}

pub async fn update_symbol_set(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(symbol_set): Json<SymbolSet>,
) -> ApiResult<Json<SymbolSet>> {
    if id != symbol_set.id {
        return Err(ApiError::BadRequest(
            "Symbol set ID in path does not match ID in body".to_string(),
        ));
    }

    if symbol_set.name.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Symbol set name cannot be empty".to_string(),
        ));
    }

    let updated_symbol_set = state
        .symbol_set_repository
        .update_symbol_set(symbol_set)
        .await?;
    Ok(Json(updated_symbol_set))
}

pub async fn delete_symbol_set(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<StatusCode, ApiError> {
    if id.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Symbol set ID cannot be empty".to_string(),
        ));
    }

    state.symbol_set_repository.delete_symbol_set(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct AddRelatedSymbolRequest {
    pub related_symbol_id: String,
}

pub async fn add_related_symbol(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<AddRelatedSymbolRequest>,
) -> ApiResult<Json<DomainSymbol>> {
    if id.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Symbol ID cannot be empty".to_string(),
        ));
    }
    if request.related_symbol_id.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Related symbol ID cannot be empty".to_string(),
        ));
    }

    let mut symbol = state.symbol_repository.get_symbol(&id).await?;

    let _ = state
        .symbol_repository
        .get_symbol(&request.related_symbol_id)
        .await?;

    symbol.related_symbols.push(request.related_symbol_id);

    let updated_symbol = state.symbol_repository.update_symbol(symbol).await?;
    Ok(Json(updated_symbol))
}
