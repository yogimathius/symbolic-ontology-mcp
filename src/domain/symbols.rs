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
    pub fn add_interpretation(&mut self, context: String, interpretation: String) {
        self.interpretations.insert(context, interpretation);
    }

    /// Add a related symbol by ID
    pub fn add_related_symbol(&mut self, symbol_id: String) {
        if !self.related_symbols.contains(&symbol_id) {
            self.related_symbols.push(symbol_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_creation() {
        let symbol = Symbol::new(
            "water".to_string(),
            "Water".to_string(),
            "dream".to_string(),
            "Symbolizes emotions and the unconscious".to_string(),
        );

        assert_eq!(symbol.id, "water");
        assert_eq!(symbol.name, "Water");
        assert_eq!(symbol.category, "dream");
        assert_eq!(
            symbol.description,
            "Symbolizes emotions and the unconscious"
        );
        assert!(symbol.interpretations.is_empty());
        assert!(symbol.related_symbols.is_empty());
    }

    #[test]
    fn test_add_interpretation() {
        let mut symbol = Symbol::new(
            "water".to_string(),
            "Water".to_string(),
            "dream".to_string(),
            "Symbolizes emotions and the unconscious".to_string(),
        );

        symbol.add_interpretation(
            "psychology".to_string(),
            "Represents the depths of the unconscious mind".to_string(),
        );

        assert_eq!(
            symbol.interpretations.get("psychology").unwrap(),
            "Represents the depths of the unconscious mind"
        );
    }

    #[test]
    fn test_add_related_symbol() {
        let mut symbol = Symbol::new(
            "water".to_string(),
            "Water".to_string(),
            "dream".to_string(),
            "Symbolizes emotions and the unconscious".to_string(),
        );

        symbol.add_related_symbol("ocean".to_string());
        symbol.add_related_symbol("river".to_string());
        // Adding duplicate should not add it again
        symbol.add_related_symbol("ocean".to_string());

        assert_eq!(symbol.related_symbols.len(), 2);
        assert!(symbol.related_symbols.contains(&"ocean".to_string()));
        assert!(symbol.related_symbols.contains(&"river".to_string()));
    }
}
