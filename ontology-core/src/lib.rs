// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

/*!
 * # Ontology Core Library
 *
 * This is the core library for the Symbol Ontology project.
 * It contains domain models, database interactions, and shared utilities.
 */

// We'll gradually move functionality from the main crate to here
// For now, just define the module structure

pub mod db;
pub mod domain;
pub mod utils;

// Re-export key components for convenient usage
pub use domain::ontology::SymbolSet;
pub use domain::symbols::Symbol;
