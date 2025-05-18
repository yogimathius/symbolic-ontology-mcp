use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::symbols::Symbol;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolSet {
    pub id: String,

    pub name: String,

    pub description: String,

    pub category: String,

    #[serde(default)]
    pub symbols: HashMap<String, Symbol>,
}

impl SymbolSet {
    pub fn new(id: String, name: String, category: String, description: String) -> Self {
        SymbolSet {
            id,
            name,
            category,
            description,
            symbols: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_symbols(mut self, symbol_ids: Vec<&str>) -> Self {
        // Note: This just stores empty placeholders - typically you'd populate
        // with real symbols later through the repository
        for id in symbol_ids {
            self.symbols.insert(
                id.to_string(),
                Symbol::new(
                    id.to_string(),
                    id.to_string(), // Using ID as name for placeholder
                    "".to_string(), // Empty category
                    "".to_string(), // Empty description
                ),
            );
        }
        self
    }

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.id.clone(), symbol);
    }

    #[allow(dead_code)]
    pub fn get_symbol(&self, id: &str) -> Option<&Symbol> {
        self.symbols.get(id)
    }

    #[allow(dead_code)]
    pub fn remove_symbol(&mut self, id: &str) -> Option<Symbol> {
        self.symbols.remove(id)
    }

    #[allow(dead_code)]
    pub fn search(&self, query: &str) -> Vec<&Symbol> {
        let query = query.to_lowercase();
        self.symbols
            .values()
            .filter(|s| {
                s.id.to_lowercase().contains(&query)
                    || s.name.to_lowercase().contains(&query)
                    || s.description.to_lowercase().contains(&query)
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn filter_by_category(&self, category: &str) -> Vec<&Symbol> {
        self.symbols
            .values()
            .filter(|s| s.category == category)
            .collect()
    }

    #[allow(dead_code)]
    pub fn get_categories(&self) -> HashSet<&str> {
        self.symbols.values().map(|s| s.category.as_str()).collect()
    }

    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.symbols.len()
    }
}
