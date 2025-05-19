// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

/*!
 * # Repository Implementation Module
 *
 * This module contains repository interfaces and their implementations
 * using PostgreSQL database as the backend.
 */

pub mod factory;
pub mod interfaces;
pub mod symbol_repository;
pub mod symbol_set_repository;

pub use factory::{PgRepositoryFactory, RepositoryFactory};
pub use interfaces::{Repository, RepositoryError, SymbolRepository, SymbolSetRepository};
pub use symbol_repository::PgSymbolRepository;
pub use symbol_set_repository::PgSymbolSetRepository;
