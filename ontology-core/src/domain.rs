// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

/*!
 * Domain models for the Symbol Ontology
 */

// Placeholder - we'll copy the actual Symbol implementation here later
pub struct Symbol {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
}

// We'll implement this fully once we migrate code
impl Symbol {
    pub fn new(id: String, name: String, category: String, description: String) -> Self {
        Self {
            id,
            name,
            category,
            description,
        }
    }
}
