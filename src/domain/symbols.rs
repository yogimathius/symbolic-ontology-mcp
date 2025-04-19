use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a symbolic entity with its properties and interpretations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Symbol {
    /// Unique identifier for the symbol
    pub id: String,

    /// Name of the symbol
    pub name: String,

    /// Primary category (dream, mythological, archetypal, etc.)
    pub category: String,

    /// Brief description of the symbol
    pub description: String,

    /// Detailed interpretations across different contexts
    pub interpretations: HashMap<String, String>,

    /// Related symbols (by ID)
    pub related_symbols: Vec<String>,

    /// Additional properties as key-value pairs
    #[serde(default)]
    pub properties: HashMap<String, String>,
}

impl Symbol {
    /// Create a new symbol with minimal fields
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

    /// Set or update the category
    pub fn with_category(mut self, category: &str) -> Self {
        self.category = category.to_string();
        self
    }

    /// Add related symbols
    pub fn with_related(mut self, related: Vec<&str>) -> Self {
        self.related_symbols = related.into_iter().map(|s| s.to_string()).collect();
        self
    }

    /// Add an interpretation for a specific context
    #[allow(dead_code)]
    pub fn add_interpretation(&mut self, context: String, interpretation: String) {
        self.interpretations.insert(context, interpretation);
    }

    /// Add a related symbol ID
    #[allow(dead_code)]
    pub fn add_related_symbol(&mut self, symbol_id: String) {
        self.related_symbols.push(symbol_id);
    }
}
