// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

/*!
 * # Database Module
 *
 * This module contains the database models, connection pool, and queries for the symbolic ontology.
 */

pub mod models;
pub mod pool;
pub mod queries;
pub mod repository;
pub mod schema;

pub use models::{Symbol, SymbolSet};
pub use pool::{DbError, create_pool, init_database};
pub use repository::{Repository, RepositoryFactory};
