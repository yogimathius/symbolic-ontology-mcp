// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

use sqlx::{PgPool, Row};
use tracing::info;

use crate::db::models::{Symbol, SymbolSet};
use crate::db::pool::{DbError, DbResult};

pub struct SymbolQueries;

impl SymbolQueries {
    pub async fn get_by_id(pool: &PgPool, id: &str) -> DbResult<Symbol> {
        sqlx::query_as::<_, Symbol>("SELECT * FROM symbols WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => DbError::NotFound,
                _ => DbError::Sqlx(e),
            })
    }

    pub async fn list(pool: &PgPool, category: Option<&str>) -> DbResult<Vec<Symbol>> {
        match category {
            Some(cat) => sqlx::query_as::<_, Symbol>("SELECT * FROM symbols WHERE category = $1")
                .bind(cat)
                .fetch_all(pool)
                .await
                .map_err(|e| DbError::Sqlx(e)),
            None => sqlx::query_as::<_, Symbol>("SELECT * FROM symbols")
                .fetch_all(pool)
                .await
                .map_err(|e| DbError::Sqlx(e)),
        }
    }

    pub async fn search(pool: &PgPool, query: &str) -> DbResult<Vec<Symbol>> {
        let search_pattern = format!("%{}%", query);
        sqlx::query_as::<_, Symbol>(
            "SELECT * FROM symbols WHERE name ILIKE $1 OR description ILIKE $1",
        )
        .bind(&search_pattern)
        .fetch_all(pool)
        .await
        .map_err(|e| DbError::Sqlx(e))
    }

    pub async fn create(pool: &PgPool, symbol: &Symbol) -> DbResult<Symbol> {
        // Check if symbol already exists
        let exists = sqlx::query("SELECT 1 FROM symbols WHERE id = $1")
            .bind(&symbol.id)
            .fetch_optional(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))?
            .is_some();

        if exists {
            return Err(DbError::Conflict(format!(
                "Symbol with ID {} already exists",
                symbol.id
            )));
        }

        // Convert JSON fields for storage
        let interpretations = serde_json::to_value(&symbol.interpretations).unwrap_or_default();
        let related_symbols = serde_json::to_value(&symbol.related_symbols).unwrap_or_default();
        let properties = serde_json::to_value(&symbol.properties).unwrap_or_default();

        sqlx::query(
            r#"
            INSERT INTO symbols (id, name, category, description, interpretations, related_symbols, properties)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(&symbol.id)
        .bind(&symbol.name)
        .bind(&symbol.category)
        .bind(&symbol.description)
        .bind(interpretations)
        .bind(related_symbols)
        .bind(properties)
        .execute(pool)
        .await
        .map_err(|e| DbError::Sqlx(e))?;

        // Return the created symbol
        Self::get_by_id(pool, &symbol.id).await
    }

    pub async fn update(pool: &PgPool, symbol: &Symbol) -> DbResult<Symbol> {
        // Check if symbol exists
        let exists = sqlx::query("SELECT 1 FROM symbols WHERE id = $1")
            .bind(&symbol.id)
            .fetch_optional(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))?
            .is_some();

        if !exists {
            return Err(DbError::NotFound);
        }

        // Convert JSON fields for storage
        let interpretations = serde_json::to_value(&symbol.interpretations).unwrap_or_default();
        let related_symbols = serde_json::to_value(&symbol.related_symbols).unwrap_or_default();
        let properties = serde_json::to_value(&symbol.properties).unwrap_or_default();

        sqlx::query(
            r#"
            UPDATE symbols 
            SET name = $2, category = $3, description = $4, 
                interpretations = $5, related_symbols = $6, properties = $7
            WHERE id = $1
            "#,
        )
        .bind(&symbol.id)
        .bind(&symbol.name)
        .bind(&symbol.category)
        .bind(&symbol.description)
        .bind(interpretations)
        .bind(related_symbols)
        .bind(properties)
        .execute(pool)
        .await
        .map_err(|e| DbError::Sqlx(e))?;

        // Return the updated symbol
        Self::get_by_id(pool, &symbol.id).await
    }

    pub async fn delete(pool: &PgPool, id: &str) -> DbResult<()> {
        // Check if symbol exists
        let exists = sqlx::query("SELECT 1 FROM symbols WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))?
            .is_some();

        if !exists {
            return Err(DbError::NotFound);
        }

        sqlx::query("DELETE FROM symbols WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))?;

        Ok(())
    }

    pub async fn seed_test_data(pool: &PgPool) -> DbResult<()> {
        info!("Seeding test data for symbols...");

        let test_symbols = vec![
            Symbol {
                id: "sun".to_string(),
                name: "Sun".to_string(),
                category: "nature".to_string(),
                description: "Celestial body at the center of our solar system".to_string(),
                interpretations: [(
                    "default".to_string(),
                    "Represents life, energy, and vitality".to_string(),
                )]
                .iter()
                .cloned()
                .collect(),
                related_symbols: vec!["light".to_string(), "day".to_string()],
                properties: [("element".to_string(), "fire".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Symbol {
                id: "moon".to_string(),
                name: "Moon".to_string(),
                category: "nature".to_string(),
                description: "Natural satellite of Earth".to_string(),
                interpretations: [(
                    "default".to_string(),
                    "Represents intuition, femininity and cycles".to_string(),
                )]
                .iter()
                .cloned()
                .collect(),
                related_symbols: vec!["night".to_string(), "tide".to_string()],
                properties: [("element".to_string(), "water".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Symbol {
                id: "light".to_string(),
                name: "Light".to_string(),
                category: "concept".to_string(),
                description: "Electromagnetic radiation visible to the human eye".to_string(),
                interpretations: [(
                    "default".to_string(),
                    "Represents knowledge, truth, and enlightenment".to_string(),
                )]
                .iter()
                .cloned()
                .collect(),
                related_symbols: vec!["sun".to_string(), "illumination".to_string()],
                properties: [("element".to_string(), "fire".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
            },
        ];

        for symbol in test_symbols {
            // Check if exists, only insert if not
            let exists = sqlx::query("SELECT 1 FROM symbols WHERE id = $1")
                .bind(&symbol.id)
                .fetch_optional(pool)
                .await
                .map_err(|e| DbError::Sqlx(e))?
                .is_some();

            if !exists {
                Self::create(pool, &symbol).await?;
                info!("Created test symbol: {}", symbol.id);
            } else {
                info!("Test symbol already exists: {}", symbol.id);
            }
        }

        info!("Seed data complete for symbols");
        Ok(())
    }
}

pub struct SymbolSetQueries;

impl SymbolSetQueries {
    pub async fn get_by_id(pool: &PgPool, id: &str) -> DbResult<SymbolSet> {
        sqlx::query_as::<_, SymbolSet>("SELECT * FROM symbol_sets WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => DbError::NotFound,
                _ => DbError::Sqlx(e),
            })
    }

    pub async fn list(pool: &PgPool, category: Option<&str>) -> DbResult<Vec<SymbolSet>> {
        match category {
            Some(cat) => {
                sqlx::query_as::<_, SymbolSet>("SELECT * FROM symbol_sets WHERE category = $1")
                    .bind(cat)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| DbError::Sqlx(e))
            }
            None => sqlx::query_as::<_, SymbolSet>("SELECT * FROM symbol_sets")
                .fetch_all(pool)
                .await
                .map_err(|e| DbError::Sqlx(e)),
        }
    }

    pub async fn search(pool: &PgPool, query: &str) -> DbResult<Vec<SymbolSet>> {
        let search_pattern = format!("%{}%", query);
        sqlx::query_as::<_, SymbolSet>(
            "SELECT * FROM symbol_sets WHERE name ILIKE $1 OR description ILIKE $1",
        )
        .bind(&search_pattern)
        .fetch_all(pool)
        .await
        .map_err(|e| DbError::Sqlx(e))
    }

    pub async fn create(pool: &PgPool, set: &SymbolSet) -> DbResult<SymbolSet> {
        // Check if set already exists
        let exists = sqlx::query("SELECT 1 FROM symbol_sets WHERE id = $1")
            .bind(&set.id)
            .fetch_optional(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))?
            .is_some();

        if exists {
            return Err(DbError::Conflict(format!(
                "SymbolSet with ID {} already exists",
                set.id
            )));
        }

        // Convert JSON field for storage
        let symbols_map = serde_json::to_value(&set.symbols_map).unwrap_or_default();

        sqlx::query(
            r#"
            INSERT INTO symbol_sets (id, name, category, description, symbols)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(&set.id)
        .bind(&set.name)
        .bind(&set.category)
        .bind(&set.description)
        .bind(symbols_map)
        .execute(pool)
        .await
        .map_err(|e| DbError::Sqlx(e))?;

        // Return the created set
        Self::get_by_id(pool, &set.id).await
    }

    pub async fn update(pool: &PgPool, set: &SymbolSet) -> DbResult<SymbolSet> {
        // Check if set exists
        let exists = sqlx::query("SELECT 1 FROM symbol_sets WHERE id = $1")
            .bind(&set.id)
            .fetch_optional(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))?
            .is_some();

        if !exists {
            return Err(DbError::NotFound);
        }

        // Convert JSON field for storage
        let symbols_map = serde_json::to_value(&set.symbols_map).unwrap_or_default();

        sqlx::query(
            r#"
            UPDATE symbol_sets 
            SET name = $2, category = $3, description = $4, symbols = $5
            WHERE id = $1
            "#,
        )
        .bind(&set.id)
        .bind(&set.name)
        .bind(&set.category)
        .bind(&set.description)
        .bind(symbols_map)
        .execute(pool)
        .await
        .map_err(|e| DbError::Sqlx(e))?;

        // Return the updated set
        Self::get_by_id(pool, &set.id).await
    }

    pub async fn delete(pool: &PgPool, id: &str) -> DbResult<()> {
        // Check if set exists
        let exists = sqlx::query("SELECT 1 FROM symbol_sets WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))?
            .is_some();

        if !exists {
            return Err(DbError::NotFound);
        }

        sqlx::query("DELETE FROM symbol_sets WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))?;

        Ok(())
    }
}
