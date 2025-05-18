use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Record not found")]
    NotFound,

    #[error("Conflict: {0}")]
    Conflict(String),
}

pub type DbResult<T> = Result<T, DbError>;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await
}

pub async fn init_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS symbols (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            category TEXT NOT NULL,
            description TEXT NOT NULL,
            interpretations JSONB DEFAULT '{}'::JSONB,
            related_symbols JSONB DEFAULT '[]'::JSONB,
            properties JSONB DEFAULT '{}'::JSONB
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS symbol_sets (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            category TEXT NOT NULL,
            description TEXT NOT NULL,
            symbols JSONB DEFAULT '{}'::JSONB
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
