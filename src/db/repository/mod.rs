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

pub use factory::PgRepositoryFactory;
pub use interfaces::{RepositoryError, SymbolRepository, SymbolSetRepository};
pub use symbol_repository::PgSymbolRepository;
pub use symbol_set_repository::PgSymbolSetRepository;
