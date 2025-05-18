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
                e => DbError::Sqlx(e),
            })
    }

    pub async fn list(pool: &PgPool, category: Option<&str>) -> DbResult<Vec<Symbol>> {
        match category {
            Some(cat) => sqlx::query_as::<_, Symbol>("SELECT * FROM symbols WHERE category = $1")
                .bind(cat)
                .fetch_all(pool)
                .await
                .map_err(DbError::Sqlx),
            None => sqlx::query_as::<_, Symbol>("SELECT * FROM symbols")
                .fetch_all(pool)
                .await
                .map_err(DbError::Sqlx),
        }
    }

    pub async fn search(pool: &PgPool, query: &str) -> DbResult<Vec<Symbol>> {
        let query_param = format!("%{}%", query.to_lowercase());

        match sqlx::query_as::<_, Symbol>(
            "SELECT * FROM symbols WHERE 
            LOWER(name) LIKE $1 OR 
            LOWER(description) LIKE $1 OR
            LOWER(id) LIKE $1",
        )
        .bind(&query_param)
        .fetch_all(pool)
        .await
        {
            Ok(results) => Ok(results),
            Err(e) => Err(DbError::Sqlx(e)),
        }
    }

    pub async fn create(pool: &PgPool, symbol: &Symbol) -> DbResult<Symbol> {
        let exists = sqlx::query("SELECT EXISTS(SELECT 1 FROM symbols WHERE id = $1)")
            .bind(&symbol.id)
            .fetch_one(pool)
            .await
            .map_err(DbError::Sqlx)?;

        let exists: bool = exists.try_get(0).map_err(DbError::Sqlx)?;
        if exists {
            return Err(DbError::Conflict(format!(
                "Symbol with ID '{}' already exists",
                symbol.id
            )));
        }

        let interpretations = sqlx::types::Json(&symbol.interpretations);
        let related_symbols = sqlx::types::Json(&symbol.related_symbols);
        let properties = sqlx::types::Json(&symbol.properties);

        sqlx::query(
            r#"
            INSERT INTO symbols (
                id, name, category, description, interpretations, related_symbols, properties
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(&symbol.id)
        .bind(&symbol.name)
        .bind(&symbol.category)
        .bind(&symbol.description)
        .bind(&interpretations)
        .bind(&related_symbols)
        .bind(&properties)
        .execute(pool)
        .await
        .map_err(DbError::Sqlx)?;

        Ok(symbol.clone())
    }

    pub async fn update(pool: &PgPool, symbol: &Symbol) -> DbResult<Symbol> {
        let exists = sqlx::query("SELECT EXISTS(SELECT 1 FROM symbols WHERE id = $1)")
            .bind(&symbol.id)
            .fetch_one(pool)
            .await
            .map_err(DbError::Sqlx)?;

        let exists: bool = exists.try_get(0).map_err(DbError::Sqlx)?;
        if !exists {
            return Err(DbError::NotFound);
        }

        let interpretations = sqlx::types::Json(&symbol.interpretations);
        let related_symbols = sqlx::types::Json(&symbol.related_symbols);
        let properties = sqlx::types::Json(&symbol.properties);

        sqlx::query(
            r#"
            UPDATE symbols SET 
                name = $2, 
                category = $3, 
                description = $4, 
                interpretations = $5, 
                related_symbols = $6, 
                properties = $7
            WHERE id = $1
            "#,
        )
        .bind(&symbol.id)
        .bind(&symbol.name)
        .bind(&symbol.category)
        .bind(&symbol.description)
        .bind(&interpretations)
        .bind(&related_symbols)
        .bind(&properties)
        .execute(pool)
        .await
        .map_err(DbError::Sqlx)?;

        Ok(symbol.clone())
    }

    pub async fn delete(pool: &PgPool, id: &str) -> DbResult<()> {
        let exists = sqlx::query("SELECT EXISTS(SELECT 1 FROM symbols WHERE id = $1)")
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(DbError::Sqlx)?;

        let exists: bool = exists.try_get(0).map_err(DbError::Sqlx)?;
        if !exists {
            return Err(DbError::NotFound);
        }

        sqlx::query("DELETE FROM symbols WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(DbError::Sqlx)?;

        Ok(())
    }

    pub async fn seed_test_data(pool: &PgPool) -> DbResult<()> {
        let test_symbols = vec![
            Symbol::new(
                "sun".to_string(),
                "Sun".to_string(),
                "nature".to_string(),
                "Celestial body at the center of our solar system".to_string(),
            )
            .with_category("nature")
            .with_related(vec!["light", "day"]),
            Symbol::new(
                "moon".to_string(),
                "Moon".to_string(),
                "nature".to_string(),
                "Natural satellite of Earth".to_string(),
            )
            .with_category("nature")
            .with_related(vec!["night", "tide"]),
            Symbol::new(
                "light".to_string(),
                "Light".to_string(),
                "concept".to_string(),
                "Electromagnetic radiation visible to the human eye".to_string(),
            )
            .with_category("concept")
            .with_related(vec!["sun", "illumination"]),
            Symbol::new(
                "dark".to_string(),
                "Darkness".to_string(),
                "concept".to_string(),
                "Absence of light".to_string(),
            )
            .with_category("concept")
            .with_related(vec!["night", "shadow"]),
            Symbol::new(
                "tree".to_string(),
                "Tree".to_string(),
                "nature".to_string(),
                "Perennial plant with an elongated stem and branches".to_string(),
            )
            .with_category("nature")
            .with_related(vec!["forest", "wood"]),
        ];

        let celestial_set = SymbolSet::new(
            "celestial".to_string(),
            "Celestial Bodies".to_string(),
            "nature".to_string(),
            "Celestial bodies and phenomena".to_string(),
        )
        .with_symbols(vec!["sun", "moon", "star"]);

        let opposites_set = SymbolSet::new(
            "opposites".to_string(),
            "Opposing Concepts".to_string(),
            "concept".to_string(),
            "Paired opposing concepts".to_string(),
        )
        .with_symbols(vec!["light", "dark", "day", "night"]);

        for symbol in &test_symbols {
            let _ = Self::create(pool, symbol).await;
        }

        let _ = SymbolSetQueries::create(pool, &celestial_set).await;
        let _ = SymbolSetQueries::create(pool, &opposites_set).await;

        info!("Test data seeded successfully");
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
                e => DbError::Sqlx(e),
            })
    }

    pub async fn list(pool: &PgPool, category: Option<&str>) -> DbResult<Vec<SymbolSet>> {
        match category {
            Some(cat) => {
                sqlx::query_as::<_, SymbolSet>("SELECT * FROM symbol_sets WHERE category = $1")
                    .bind(cat)
                    .fetch_all(pool)
                    .await
                    .map_err(DbError::Sqlx)
            }
            None => sqlx::query_as::<_, SymbolSet>("SELECT * FROM symbol_sets")
                .fetch_all(pool)
                .await
                .map_err(DbError::Sqlx),
        }
    }

    pub async fn search(pool: &PgPool, query: &str) -> DbResult<Vec<SymbolSet>> {
        let query_param = format!("%{}%", query.to_lowercase());

        sqlx::query_as::<_, SymbolSet>(
            "SELECT * FROM symbol_sets WHERE 
            LOWER(name) LIKE $1 OR 
            LOWER(description) LIKE $1 OR
            LOWER(id) LIKE $1",
        )
        .bind(&query_param)
        .fetch_all(pool)
        .await
        .map_err(DbError::Sqlx)
    }

    pub async fn create(pool: &PgPool, set: &SymbolSet) -> DbResult<SymbolSet> {
        let exists = sqlx::query("SELECT EXISTS(SELECT 1 FROM symbol_sets WHERE id = $1)")
            .bind(&set.id)
            .fetch_one(pool)
            .await
            .map_err(DbError::Sqlx)?;

        let exists: bool = exists.try_get(0).map_err(DbError::Sqlx)?;
        if exists {
            return Err(DbError::Conflict(format!(
                "SymbolSet with ID '{}' already exists",
                set.id
            )));
        }

        let symbols_json = sqlx::types::Json(&set.symbols_map);

        sqlx::query(
            r#"
            INSERT INTO symbol_sets (
                id, name, category, description, symbols
            ) VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(&set.id)
        .bind(&set.name)
        .bind(&set.category)
        .bind(&set.description)
        .bind(&symbols_json)
        .execute(pool)
        .await
        .map_err(DbError::Sqlx)?;

        Ok(set.clone())
    }

    pub async fn update(pool: &PgPool, set: &SymbolSet) -> DbResult<SymbolSet> {
        let exists = sqlx::query("SELECT EXISTS(SELECT 1 FROM symbol_sets WHERE id = $1)")
            .bind(&set.id)
            .fetch_one(pool)
            .await
            .map_err(DbError::Sqlx)?;

        let exists: bool = exists.try_get(0).map_err(DbError::Sqlx)?;
        if !exists {
            return Err(DbError::NotFound);
        }

        let symbols_json = sqlx::types::Json(&set.symbols_map);

        sqlx::query(
            r#"
            UPDATE symbol_sets SET 
                name = $2, 
                category = $3, 
                description = $4, 
                symbols = $5
            WHERE id = $1
            "#,
        )
        .bind(&set.id)
        .bind(&set.name)
        .bind(&set.category)
        .bind(&set.description)
        .bind(&symbols_json)
        .execute(pool)
        .await
        .map_err(DbError::Sqlx)?;

        Ok(set.clone())
    }

    pub async fn delete(pool: &PgPool, id: &str) -> DbResult<()> {
        let exists = sqlx::query("SELECT EXISTS(SELECT 1 FROM symbol_sets WHERE id = $1)")
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(DbError::Sqlx)?;

        let exists: bool = exists.try_get(0).map_err(DbError::Sqlx)?;
        if !exists {
            return Err(DbError::NotFound);
        }

        sqlx::query("DELETE FROM symbol_sets WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(DbError::Sqlx)?;

        Ok(())
    }
}
