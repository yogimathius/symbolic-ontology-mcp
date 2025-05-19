// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

/*!
 * # Domain Module
 *
 * The domain module contains the core business logic and models for the symbolic reasoning engine.
 * This includes:
 *
 * - **Symbol**: Represents a symbolic entity with interpretations and relationships
 * - **SymbolSet**: A collection of related symbols organized into an ontology
 */

pub mod ontology;
pub mod symbols;

pub use ontology::SymbolSet;
pub use symbols::Symbol;
