use crate::common::fixtures;
use ontology_core::domain::SymbolSet;

#[test]
fn test_symbolset_creation() {
    let symbol_set = SymbolSet::new(
        "dream-symbols".to_string(),
        "Dream Symbols".to_string(),
        "dream".to_string(),
        "Common symbols in dreams".to_string(),
    );

    assert_eq!(symbol_set.id, "dream-symbols");
    assert_eq!(symbol_set.name, "Dream Symbols");
    assert_eq!(symbol_set.category, "dream");
    assert_eq!(symbol_set.description, "Common symbols in dreams");
    assert_eq!(symbol_set.count(), 0);
}

#[test]
fn test_symbolset_add_get_symbol() {
    let mut symbol_set = SymbolSet::new(
        "dream-symbols".to_string(),
        "Dream Symbols".to_string(),
        "dream".to_string(),
        "Common symbols in dreams".to_string(),
    );

    let symbol = fixtures::create_test_symbol("water", "Water", "dream");
    symbol_set.add_symbol(symbol);

    assert_eq!(symbol_set.count(), 1);

    let retrieved = symbol_set.get_symbol("water");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name, "Water");
}

#[test]
fn test_symbolset_remove_symbol() {
    let mut symbol_set = SymbolSet::new(
        "dream-symbols".to_string(),
        "Dream Symbols".to_string(),
        "dream".to_string(),
        "Common symbols in dreams".to_string(),
    );

    let symbol = fixtures::create_test_symbol("water", "Water", "dream");
    symbol_set.add_symbol(symbol);
    assert_eq!(symbol_set.count(), 1);

    let removed = symbol_set.remove_symbol("water");
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().id, "water");

    assert_eq!(symbol_set.count(), 0);
    assert!(symbol_set.get_symbol("water").is_none());
}

#[test]
fn test_symbolset_search() {
    let symbol_set =
        fixtures::create_test_symbol_set("test-set", "Test Set", "test", "Test symbol set");

    let results = symbol_set.search("water");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "water");

    let results = symbol_set.search("mountain");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "mountain");

    let results = symbol_set.search("xyz");
    assert_eq!(results.len(), 0);
}

#[test]
fn test_symbolset_filter_by_category() {
    let mut symbol_set = SymbolSet::new(
        "mixed-symbols".to_string(),
        "Mixed Symbols".to_string(),
        "mixed".to_string(),
        "Mixed category symbols".to_string(),
    );

    symbol_set.add_symbol(fixtures::create_test_symbol("water", "Water", "dream"));
    symbol_set.add_symbol(fixtures::create_test_symbol(
        "dragon",
        "Dragon",
        "mythological",
    ));
    symbol_set.add_symbol(fixtures::create_test_symbol("tree", "Tree", "dream"));

    let dream_symbols = symbol_set.filter_by_category("dream");
    assert_eq!(dream_symbols.len(), 2);

    let myth_symbols = symbol_set.filter_by_category("mythological");
    assert_eq!(myth_symbols.len(), 1);
    assert_eq!(myth_symbols[0].id, "dragon");

    let empty_result = symbol_set.filter_by_category("nonexistent");
    assert_eq!(empty_result.len(), 0);
}
