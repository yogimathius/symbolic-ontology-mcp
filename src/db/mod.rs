// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

/*!
 * # Database Module
 *
 * This module re-exports the database functionality from ontology-core,
 * providing a unified interface for database operations.
 */

// Re-export all database functionality from ontology-core
pub use ontology_core::db::*;

// Specific re-exports for convenience
pub use ontology_core::db::models::Symbol;
pub use ontology_core::db::models::SymbolSet;
pub use ontology_core::db::pool::create_pool;
pub use ontology_core::db::pool::init_database;
pub use ontology_core::db::pool::DbError;
pub use ontology_core::db::repository::interfaces::SymbolRepository;
pub use ontology_core::db::repository::interfaces::SymbolSetRepository;
pub use ontology_core::db::repository::PgRepositoryFactory;
pub use ontology_core::db::repository::PgSymbolRepository;
pub use ontology_core::db::repository::PgSymbolSetRepository;
pub use ontology_core::db::repository::Repository;
pub use ontology_core::db::repository::RepositoryError;
pub use ontology_core::db::repository::RepositoryFactory;

// Additional database-specific functionality can be added here
