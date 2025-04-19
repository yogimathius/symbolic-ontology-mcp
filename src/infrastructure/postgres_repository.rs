use crate::domain::{
    RepositoryError, RepositoryFactory, RepositoryResult, Symbol, SymbolRepository, SymbolSet,
    SymbolSetRepository,
};
use async_trait::async_trait;
use serde_json::Value;
use sqlx::{PgPool, Row, postgres::PgPoolOptions};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info};

/// PostgreSQL implementation of the repositories
pub struct PostgresRepositoryFactory {
    pool: PgPool,
}

impl PostgresRepositoryFactory {
    /// Create a new PostgreSQL repository factory
    pub async fn new(database_url: &str) -> Result<Self, RepositoryError> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(5))
            .connect(database_url)
            .await
            .map_err(|e| {
                RepositoryError::Internal(format!("Failed to connect to database: {}", e))
            })?;

        // Check if pgvector extension exists
        Self::init_database(&pool).await?;

        Ok(Self { pool })
    }

    /// Initialize the database with required schemas and tables
    async fn init_database(pool: &PgPool) -> Result<(), RepositoryError> {
        // Create symbols table if it doesn't exist
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
        .await
        .map_err(|e| RepositoryError::Internal(format!("Failed to create symbols table: {}", e)))?;

        // Create symbol_sets table if it doesn't exist
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
        .await
        .map_err(|e| {
            RepositoryError::Internal(format!("Failed to create symbol_sets table: {}", e))
        })?;

        info!("Database initialized successfully");
        Ok(())
    }

    /// Seed the database with test data
    pub async fn with_test_data(self) -> Result<Self, RepositoryError> {
        let symbol_repo = PostgresSymbolRepository::new(self.pool.clone());
        let symbol_set_repo = PostgresSymbolSetRepository::new(self.pool.clone());

        // Define test symbols
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

        // Create test symbol sets with symbols
        let mut celestial_set = SymbolSet::new(
            "celestial".to_string(),
            "Celestial Bodies".to_string(),
            "nature".to_string(),
            "Celestial bodies and phenomena".to_string(),
        );

        let mut opposites_set = SymbolSet::new(
            "opposites".to_string(),
            "Opposing Concepts".to_string(),
            "concept".to_string(),
            "Paired opposing concepts".to_string(),
        );

        // Insert test symbols
        for symbol in test_symbols {
            // Add to appropriate symbol set before saving
            if symbol.id == "sun" || symbol.id == "moon" {
                celestial_set.add_symbol(symbol.clone());
            } else if symbol.id == "light" || symbol.id == "dark" {
                opposites_set.add_symbol(symbol.clone());
            }

            let _ = symbol_repo.create_symbol(symbol).await;
        }

        // Insert the symbol sets
        let _ = symbol_set_repo.create_symbol_set(celestial_set).await;
        let _ = symbol_set_repo.create_symbol_set(opposites_set).await;

        info!("Test data seeded successfully");
        Ok(self)
    }
}

impl RepositoryFactory for PostgresRepositoryFactory {
    fn create_symbol_repository(&self) -> Arc<dyn SymbolRepository> {
        Arc::new(PostgresSymbolRepository::new(self.pool.clone()))
    }

    fn create_symbol_set_repository(&self) -> Arc<dyn SymbolSetRepository> {
        Arc::new(PostgresSymbolSetRepository::new(self.pool.clone()))
    }
}

/// PostgreSQL implementation of SymbolRepository
struct PostgresSymbolRepository {
    pool: PgPool,
}

impl PostgresSymbolRepository {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Helper to convert a database row to a Symbol
    async fn row_to_symbol(&self, row: sqlx::postgres::PgRow) -> Result<Symbol, RepositoryError> {
        let id: String = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let category: String = row.try_get("category")?;
        let description: String = row.try_get("description")?;

        let interpretations_json: Value = row.try_get("interpretations")?;
        let related_json: Value = row.try_get("related_symbols")?;
        let properties_json: Value = row.try_get("properties")?;

        let interpretations: HashMap<String, String> =
            serde_json::from_value(interpretations_json)?;
        let related_symbols: Vec<String> = serde_json::from_value(related_json)?;
        let properties: HashMap<String, String> = serde_json::from_value(properties_json)?;

        let symbol = Symbol {
            id,
            name,
            category,
            description,
            interpretations,
            related_symbols,
            properties,
        };

        Ok(symbol)
    }
}

#[async_trait]
impl SymbolRepository for PostgresSymbolRepository {
    async fn get_symbol(&self, id: &str) -> RepositoryResult<Symbol> {
        let row = sqlx::query("SELECT * FROM symbols WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        self.row_to_symbol(row).await
    }

    async fn list_symbols(&self, category: Option<&str>) -> RepositoryResult<Vec<Symbol>> {
        let rows = match category {
            Some(cat) => {
                sqlx::query("SELECT * FROM symbols WHERE category = $1")
                    .bind(cat)
                    .fetch_all(&self.pool)
                    .await?
            }
            None => {
                sqlx::query("SELECT * FROM symbols")
                    .fetch_all(&self.pool)
                    .await?
            }
        };

        let mut symbols = Vec::new();
        for row in rows {
            symbols.push(self.row_to_symbol(row).await?);
        }

        Ok(symbols)
    }

    async fn search_symbols(&self, query: &str) -> RepositoryResult<Vec<Symbol>> {
        // Basic text search implementation
        let query_param = format!("%{}%", query.to_lowercase());
        let rows = sqlx::query(
            "SELECT * FROM symbols WHERE LOWER(id) LIKE $1 OR LOWER(description) LIKE $1",
        )
        .bind(&query_param)
        .fetch_all(&self.pool)
        .await?;

        let mut symbols = Vec::new();
        for row in rows {
            symbols.push(self.row_to_symbol(row).await?);
        }

        Ok(symbols)
    }

    async fn create_symbol(&self, symbol: Symbol) -> RepositoryResult<Symbol> {
        // Check if symbol already exists
        let exists = sqlx::query("SELECT EXISTS(SELECT 1 FROM symbols WHERE id = $1)")
            .bind(&symbol.id)
            .fetch_one(&self.pool)
            .await?;

        let exists: bool = exists.try_get(0)?;
        if exists {
            return Err(RepositoryError::Conflict(format!(
                "Symbol already exists: {}",
                symbol.id
            )));
        }

        // Serialize fields to JSON
        let interpretations_json = serde_json::to_value(&symbol.interpretations)?;
        let related_json = serde_json::to_value(&symbol.related_symbols)?;
        let properties_json = serde_json::to_value(&symbol.properties)?;

        // Insert the symbol
        sqlx::query(
            r#"
            INSERT INTO symbols (
                id, name, category, description, 
                interpretations, related_symbols, properties
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(&symbol.id)
        .bind(&symbol.name)
        .bind(&symbol.category)
        .bind(&symbol.description)
        .bind(&interpretations_json)
        .bind(&related_json)
        .bind(&properties_json)
        .execute(&self.pool)
        .await?;

        Ok(symbol)
    }

    async fn update_symbol(&self, symbol: Symbol) -> RepositoryResult<Symbol> {
        // Check if symbol exists
        let exists = sqlx::query("SELECT EXISTS(SELECT 1 FROM symbols WHERE id = $1)")
            .bind(&symbol.id)
            .fetch_one(&self.pool)
            .await?;

        let exists: bool = exists.try_get(0)?;
        if !exists {
            return Err(RepositoryError::NotFound(format!(
                "Symbol not found: {}",
                symbol.id
            )));
        }

        // Serialize fields to JSON
        let interpretations_json = serde_json::to_value(&symbol.interpretations)?;
        let related_json = serde_json::to_value(&symbol.related_symbols)?;
        let properties_json = serde_json::to_value(&symbol.properties)?;

        // Update the symbol
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
        .bind(&interpretations_json)
        .bind(&related_json)
        .bind(&properties_json)
        .execute(&self.pool)
        .await?;

        Ok(symbol)
    }

    async fn delete_symbol(&self, id: &str) -> RepositoryResult<()> {
        // Check if symbol exists
        let exists = sqlx::query("SELECT EXISTS(SELECT 1 FROM symbols WHERE id = $1)")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        let exists: bool = exists.try_get(0)?;
        if !exists {
            return Err(RepositoryError::NotFound(format!(
                "Symbol not found: {}",
                id
            )));
        }

        // Delete the symbol
        sqlx::query("DELETE FROM symbols WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

/// PostgreSQL implementation of SymbolSetRepository
struct PostgresSymbolSetRepository {
    pool: PgPool,
    symbol_repo: PostgresSymbolRepository,
}

impl PostgresSymbolSetRepository {
    fn new(pool: PgPool) -> Self {
        Self {
            pool: pool.clone(),
            symbol_repo: PostgresSymbolRepository::new(pool),
        }
    }

    /// Helper to convert a database row to a SymbolSet
    async fn row_to_symbol_set(
        &self,
        row: sqlx::postgres::PgRow,
    ) -> Result<SymbolSet, RepositoryError> {
        let id: String = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let category: String = row.try_get("category")?;
        let description: String = row.try_get("description")?;

        let symbols_json: Value = row.try_get("symbols")?;
        let symbol_ids_map: HashMap<String, Value> = serde_json::from_value(symbols_json)?;

        // Create empty symbol set
        let mut symbol_set = SymbolSet::new(id, name, category, description);

        // Load symbols from the symbol repository
        for symbol_id in symbol_ids_map.keys() {
            // Try to get the symbol from the repository
            if let Ok(symbol) = self.symbol_repo.get_symbol(symbol_id).await {
                symbol_set.add_symbol(symbol);
            }
        }

        Ok(symbol_set)
    }
}

#[async_trait]
impl SymbolSetRepository for PostgresSymbolSetRepository {
    async fn get_symbol_set(&self, id: &str) -> RepositoryResult<SymbolSet> {
        let row = sqlx::query("SELECT * FROM symbol_sets WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        self.row_to_symbol_set(row).await
    }

    async fn list_symbol_sets(&self, category: Option<&str>) -> RepositoryResult<Vec<SymbolSet>> {
        let rows = match category {
            Some(cat) => {
                sqlx::query("SELECT * FROM symbol_sets WHERE category = $1")
                    .bind(cat)
                    .fetch_all(&self.pool)
                    .await?
            }
            None => {
                sqlx::query("SELECT * FROM symbol_sets")
                    .fetch_all(&self.pool)
                    .await?
            }
        };

        let mut sets = Vec::new();
        for row in rows {
            sets.push(self.row_to_symbol_set(row).await?);
        }

        Ok(sets)
    }

    async fn search_symbol_sets(&self, query: &str) -> RepositoryResult<Vec<SymbolSet>> {
        // Basic text search implementation
        let query_param = format!("%{}%", query.to_lowercase());
        let rows = sqlx::query(
            "SELECT * FROM symbol_sets WHERE LOWER(id) LIKE $1 OR LOWER(description) LIKE $1",
        )
        .bind(&query_param)
        .fetch_all(&self.pool)
        .await?;

        let mut sets = Vec::new();
        for row in rows {
            sets.push(self.row_to_symbol_set(row).await?);
        }

        Ok(sets)
    }

    async fn create_symbol_set(&self, set: SymbolSet) -> RepositoryResult<SymbolSet> {
        // Check if symbol set already exists
        let exists = sqlx::query("SELECT EXISTS(SELECT 1 FROM symbol_sets WHERE id = $1)")
            .bind(&set.id)
            .fetch_one(&self.pool)
            .await?;

        let exists: bool = exists.try_get(0)?;
        if exists {
            return Err(RepositoryError::Conflict(format!(
                "SymbolSet already exists: {}",
                set.id
            )));
        }

        // Create a map of symbol IDs
        let mut symbols_map = HashMap::new();
        for (id, _) in &set.symbols {
            symbols_map.insert(id.clone(), serde_json::Value::Null);
        }

        // Serialize symbols to JSON
        let symbols_json = serde_json::to_value(&symbols_map)?;

        // Insert the symbol set
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
        .execute(&self.pool)
        .await?;

        Ok(set)
    }

    async fn update_symbol_set(&self, set: SymbolSet) -> RepositoryResult<SymbolSet> {
        // Check if symbol set exists
        let exists = sqlx::query("SELECT EXISTS(SELECT 1 FROM symbol_sets WHERE id = $1)")
            .bind(&set.id)
            .fetch_one(&self.pool)
            .await?;

        let exists: bool = exists.try_get(0)?;
        if !exists {
            return Err(RepositoryError::NotFound(format!(
                "SymbolSet not found: {}",
                set.id
            )));
        }

        // Create a map of symbol IDs
        let mut symbols_map = HashMap::new();
        for (id, _) in &set.symbols {
            symbols_map.insert(id.clone(), serde_json::Value::Null);
        }

        // Serialize symbols to JSON
        let symbols_json = serde_json::to_value(&symbols_map)?;

        // Update the symbol set
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
        .execute(&self.pool)
        .await?;

        Ok(set)
    }

    async fn delete_symbol_set(&self, id: &str) -> RepositoryResult<()> {
        // Check if symbol set exists
        let exists = sqlx::query("SELECT EXISTS(SELECT 1 FROM symbol_sets WHERE id = $1)")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        let exists: bool = exists.try_get(0)?;
        if !exists {
            return Err(RepositoryError::NotFound(format!(
                "SymbolSet not found: {}",
                id
            )));
        }

        // Delete the symbol set
        sqlx::query("DELETE FROM symbol_sets WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

/// Extended repository trait that can be added later for vector search
pub trait VectorSymbolRepository: SymbolRepository {
    /// Set an embedding vector for a symbol
    async fn set_symbol_embedding(&self, id: &str, embedding: Vec<f32>) -> RepositoryResult<()>;

    /// Find symbols similar to the provided embedding vector
    async fn find_similar_symbols(
        &self,
        embedding: &[f32],
        limit: usize,
    ) -> RepositoryResult<Vec<Symbol>>;
}
