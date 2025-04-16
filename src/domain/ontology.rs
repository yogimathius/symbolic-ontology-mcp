use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::symbols::Symbol;

/// Represents a collection of symbols organized into an ontology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolSet {
    /// Unique identifier for the symbol set
    pub id: String,

    /// Name of the symbol set
    pub name: String,

    /// Description of the symbol set
    pub description: String,

    /// Category of the symbol set (e.g., "dream", "mythological")
    pub category: String,

    /// Symbols contained in this set, indexed by ID
    #[serde(default)]
    pub symbols: HashMap<String, Symbol>,
}

impl SymbolSet {
    /// Create a new empty symbol set
    pub fn new(id: String, name: String, category: String, description: String) -> Self {
        SymbolSet {
            id,
            name,
            category,
            description,
            symbols: HashMap::new(),
        }
    }

    /// Create a new symbol set with initial symbols
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

    /// Add a symbol to the set
    pub fn add_symbol(&mut self, symbol: Symbol) -> Option<Symbol> {
        self.symbols.insert(symbol.id.clone(), symbol)
    }

    /// Get a symbol by ID
    pub fn get_symbol(&self, id: &str) -> Option<&Symbol> {
        self.symbols.get(id)
    }

    /// Remove a symbol by ID
    pub fn remove_symbol(&mut self, id: &str) -> Option<Symbol> {
        self.symbols.remove(id)
    }

    /// Search for symbols matching a query in name or description
    pub fn search(&self, query: &str) -> Vec<&Symbol> {
        let query = query.to_lowercase();
        self.symbols
            .values()
            .filter(|s| {
                s.name.to_lowercase().contains(&query)
                    || s.description.to_lowercase().contains(&query)
            })
            .collect()
    }

    /// Filter symbols by category
    pub fn filter_by_category(&self, category: &str) -> Vec<&Symbol> {
        self.symbols
            .values()
            .filter(|s| s.category == category)
            .collect()
    }

    /// Get all unique categories in this symbol set
    pub fn get_categories(&self) -> HashSet<&str> {
        self.symbols.values().map(|s| s.category.as_str()).collect()
    }

    /// Count the symbols in this set
    pub fn count(&self) -> usize {
        self.symbols.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_symbol(id: &str, name: &str, category: &str) -> Symbol {
        Symbol::new(
            id.to_string(),
            name.to_string(),
            category.to_string(),
            format!("Description for {}", name),
        )
    }

    #[test]
    fn test_symbolset_creation() {
        let symbol_set = SymbolSet::new(
            "dream-symbols".to_string(),
            "Dream Symbols".to_string(),
            "dream".to_string(),
            "Common symbols appearing in dreams".to_string(),
        );

        assert_eq!(symbol_set.id, "dream-symbols");
        assert_eq!(symbol_set.name, "Dream Symbols");
        assert_eq!(symbol_set.description, "Common symbols appearing in dreams");
        assert_eq!(symbol_set.count(), 0);
    }

    #[test]
    fn test_add_and_get_symbol() {
        let mut symbol_set = SymbolSet::new(
            "dream-symbols".to_string(),
            "Dream Symbols".to_string(),
            "dream".to_string(),
            "Common symbols appearing in dreams".to_string(),
        );

        let water = create_test_symbol("water", "Water", "dream");
        let fire = create_test_symbol("fire", "Fire", "dream");

        symbol_set.add_symbol(water);
        symbol_set.add_symbol(fire);

        assert_eq!(symbol_set.count(), 2);

        let retrieved = symbol_set.get_symbol("water").unwrap();
        assert_eq!(retrieved.name, "Water");

        // Test replacing a symbol
        let new_water = create_test_symbol("water", "Water (Updated)", "dream");
        symbol_set.add_symbol(new_water);

        // Should still have 2 symbols
        assert_eq!(symbol_set.count(), 2);

        // But the water symbol should be updated
        let updated = symbol_set.get_symbol("water").unwrap();
        assert_eq!(updated.name, "Water (Updated)");
    }

    #[test]
    fn test_remove_symbol() {
        let mut symbol_set = SymbolSet::new(
            "dream-symbols".to_string(),
            "Dream Symbols".to_string(),
            "dream".to_string(),
            "Common symbols appearing in dreams".to_string(),
        );

        symbol_set.add_symbol(create_test_symbol("water", "Water", "dream"));
        symbol_set.add_symbol(create_test_symbol("fire", "Fire", "dream"));

        assert_eq!(symbol_set.count(), 2);

        let removed = symbol_set.remove_symbol("water").unwrap();
        assert_eq!(removed.name, "Water");
        assert_eq!(symbol_set.count(), 1);
        assert!(symbol_set.get_symbol("water").is_none());
    }

    #[test]
    fn test_search() {
        let mut symbol_set = SymbolSet::new(
            "dream-symbols".to_string(),
            "Dream Symbols".to_string(),
            "dream".to_string(),
            "Common symbols appearing in dreams".to_string(),
        );

        symbol_set.add_symbol(create_test_symbol("water", "Water", "dream"));
        symbol_set.add_symbol(create_test_symbol("fire", "Fire", "dream"));
        symbol_set.add_symbol(create_test_symbol("ocean", "Ocean", "dream"));

        let results = symbol_set.search("wat");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "water");

        // Search is case-insensitive
        let results = symbol_set.search("WATER");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "water");
    }

    #[test]
    fn test_filter_by_category() {
        let mut symbol_set = SymbolSet::new(
            "mixed-symbols".to_string(),
            "Mixed Symbols".to_string(),
            "mixed".to_string(),
            "Mixed symbol categories".to_string(),
        );

        symbol_set.add_symbol(create_test_symbol("water", "Water", "dream"));
        symbol_set.add_symbol(create_test_symbol("fire", "Fire", "dream"));
        symbol_set.add_symbol(create_test_symbol("dragon", "Dragon", "mythological"));
        symbol_set.add_symbol(create_test_symbol("phoenix", "Phoenix", "mythological"));

        let dream_symbols = symbol_set.filter_by_category("dream");
        assert_eq!(dream_symbols.len(), 2);

        let myth_symbols = symbol_set.filter_by_category("mythological");
        assert_eq!(myth_symbols.len(), 2);

        // Check empty result for non-existent category
        let empty = symbol_set.filter_by_category("nonexistent");
        assert_eq!(empty.len(), 0);
    }
}
