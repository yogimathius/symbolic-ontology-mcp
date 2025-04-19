use dream_ontology_mcp::domain::Symbol;

#[test]
fn test_symbol_creation() {
    let symbol = Symbol::new(
        "water".to_string(),
        "Water".to_string(),
        "dream".to_string(),
        "Symbol of the unconscious".to_string(),
    );

    assert_eq!(symbol.id, "water");
    assert_eq!(symbol.name, "Water");
    assert_eq!(symbol.category, "dream");
    assert_eq!(symbol.description, "Symbol of the unconscious");
    assert!(symbol.related_symbols.is_empty());
    assert!(symbol.interpretations.is_empty());
}

#[test]
fn test_symbol_interpretations() {
    let mut symbol = Symbol::new(
        "water".to_string(),
        "Water".to_string(),
        "dream".to_string(),
        "Symbol of the unconscious".to_string(),
    );

    symbol.add_interpretation(
        "psychological".to_string(),
        "Represents emotions and the unconscious mind".to_string(),
    );
    symbol.add_interpretation(
        "spiritual".to_string(),
        "Symbol of purification and renewal".to_string(),
    );

    assert_eq!(symbol.interpretations.len(), 2);
    assert_eq!(
        symbol.interpretations.get("psychological").unwrap(),
        "Represents emotions and the unconscious mind"
    );
    assert_eq!(
        symbol.interpretations.get("spiritual").unwrap(),
        "Symbol of purification and renewal"
    );
}

#[test]
fn test_symbol_related_symbols() {
    let mut symbol = Symbol::new(
        "water".to_string(),
        "Water".to_string(),
        "dream".to_string(),
        "Symbol of the unconscious".to_string(),
    );

    symbol.add_related_symbol("ocean".to_string());
    symbol.add_related_symbol("river".to_string());
    symbol.add_related_symbol("lake".to_string());

    assert_eq!(symbol.related_symbols.len(), 3);
    assert!(symbol.related_symbols.contains(&"ocean".to_string()));
    assert!(symbol.related_symbols.contains(&"river".to_string()));
    assert!(symbol.related_symbols.contains(&"lake".to_string()));
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
