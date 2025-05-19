// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Symbol {
    pub id: String,

    pub name: String,

    pub category: String,

    pub description: String,

    pub interpretations: HashMap<String, String>,

    pub related_symbols: Vec<String>,

    #[serde(default)]
    pub properties: HashMap<String, String>,
}

impl Symbol {
    #[allow(dead_code)]
    pub fn new(id: String, name: String, category: String, description: String) -> Self {
        Symbol {
            id,
            name,
            category,
            description,
            interpretations: HashMap::new(),
            related_symbols: Vec::new(),
            properties: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_category(mut self, category: &str) -> Self {
        self.category = category.to_string();
        self
    }

    #[allow(dead_code)]
    pub fn with_related(mut self, related: Vec<&str>) -> Self {
        self.related_symbols = related.into_iter().map(|s| s.to_string()).collect();
        self
    }

    #[allow(dead_code)]
    pub fn add_interpretation(&mut self, context: String, interpretation: String) {
        self.interpretations.insert(context, interpretation);
    }

    #[allow(dead_code)]
    pub fn add_related_symbol(&mut self, symbol_id: String) {
        self.related_symbols.push(symbol_id);
    }
}
