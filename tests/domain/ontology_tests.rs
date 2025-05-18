use dream_ontology_mcp::domain::{Symbol, SymbolSet};

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

    let new_water = create_test_symbol("water", "Water (Updated)", "dream");
    symbol_set.add_symbol(new_water);

    assert_eq!(symbol_set.count(), 2);

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

    let empty = symbol_set.filter_by_category("nonexistent");
    assert_eq!(empty.len(), 0);
}
