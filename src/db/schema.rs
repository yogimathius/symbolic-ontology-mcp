use sqlx::PgPool;
use tracing::info;

use crate::db::pool::DbError;
use crate::db::pool::DbResult;

#[allow(dead_code)]
const CREATE_SYMBOLS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS symbols (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    category TEXT NOT NULL,
    description TEXT NOT NULL,
    interpretations JSONB DEFAULT '{}'::JSONB,
    related_symbols JSONB DEFAULT '[]'::JSONB,
    properties JSONB DEFAULT '{}'::JSONB
)
"#;

#[allow(dead_code)]
const CREATE_SYMBOL_SETS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS symbol_sets (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    category TEXT NOT NULL,
    description TEXT NOT NULL,
    symbols JSONB DEFAULT '{}'::JSONB
)
"#;

#[allow(dead_code)]
const CREATE_CATEGORY_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS idx_symbols_category ON symbols (category)
"#;

#[allow(dead_code)]
const CREATE_TEXT_SEARCH_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS idx_symbols_text_search 
ON symbols USING GIN ((to_tsvector('english', name || ' ' || description)))
"#;

#[allow(dead_code)]
pub async fn init_schema(pool: &PgPool) -> DbResult<()> {
    sqlx::query(CREATE_SYMBOLS_TABLE)
        .execute(pool)
        .await
        .map_err(|e| DbError::Sqlx(e))?;

    sqlx::query(CREATE_SYMBOL_SETS_TABLE)
        .execute(pool)
        .await
        .map_err(|e| DbError::Sqlx(e))?;

    sqlx::query(CREATE_CATEGORY_INDEX)
        .execute(pool)
        .await
        .map_err(|e| DbError::Sqlx(e))?;

    match sqlx::query(CREATE_TEXT_SEARCH_INDEX).execute(pool).await {
        Ok(_) => info!("Created text search index"),
        Err(e) => info!("Could not create text search index: {}", e),
    }

    info!("Database schema initialized successfully");
    Ok(())
}

#[allow(dead_code)]
pub async fn run_migrations(pool: &PgPool) -> DbResult<()> {
    init_schema(pool).await
}
