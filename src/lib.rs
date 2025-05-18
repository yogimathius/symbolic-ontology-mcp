/*!
 * # Dream Ontology MCP Library
 *
 * This library implements a symbolic reasoning engine for dream interpretation
 * and symbolic analysis through the Model Context Protocol (MCP).
 *
 * ## Core Components
 *
 * - **Domain Models**: Define the core business objects like `Symbol` and `SymbolSet`
 * - **MCP Implementation**: Protocol-compliant methods for symbolic reasoning
 * - **Database**: Direct PostgreSQL access for data persistence
 * - **API**: HTTP endpoint handlers and routing for the Axum server
 *
 * ## Usage Example
 *
 * ```rust,no_run
 * use dream_ontology_mcp::db::{pool, queries::SymbolQueries, models::Symbol};
 * use sqlx::PgPool;
 *
 * # async fn example() -> Result<(), Box<dyn std::error::Error>> {
 * // Create a database connection pool
 * let db_pool = pool::create_pool("postgres://postgres:postgres@localhost/dream_ontology").await?;
 *
 * // Use the database directly
 * let symbol = SymbolQueries::get_by_id(&db_pool, "water").await?;
 * println!("Found symbol: {}", symbol.name);
 * # Ok(())
 * # }
 * ```
 */

pub mod domain;

pub mod mcp;

pub mod db;

pub mod api;

pub mod logging;

mod utils;

pub mod config;
