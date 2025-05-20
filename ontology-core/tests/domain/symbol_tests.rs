use ontology_core::domain::Symbol;

#[test]
fn test_symbol_creation() {
    let id = "water".to_string();
    let name = "Water".to_string();
    let category = "nature".to_string();
    let description = "Symbol of life and emotion".to_string();

    let symbol = Symbol::new(
        id.clone(),
        name.clone(),
        category.clone(),
        description.clone(),
    );

    assert_eq!(symbol.id, id);
    assert_eq!(symbol.name, name);
    assert_eq!(symbol.category, category);
    assert_eq!(symbol.description, description);
    assert!(symbol.interpretations.is_empty());
    assert!(symbol.related_symbols.is_empty());
    assert!(symbol.properties.is_empty());
}

#[test]
fn test_symbol_with_category() {
    let symbol = Symbol::new(
        "fire".to_string(),
        "Fire".to_string(),
        "element".to_string(),
        "Symbol of transformation".to_string(),
    );

    let updated = symbol.with_category("nature");

    assert_eq!(updated.category, "nature");
}

#[test]
fn test_symbol_with_related() {
    let symbol = Symbol::new(
        "ocean".to_string(),
        "Ocean".to_string(),
        "nature".to_string(),
        "Vast body of water".to_string(),
    );

    let updated = symbol.with_related(vec!["water", "wave", "sea"]);

    assert_eq!(updated.related_symbols.len(), 3);
    assert!(updated.related_symbols.contains(&"water".to_string()));
    assert!(updated.related_symbols.contains(&"wave".to_string()));
    assert!(updated.related_symbols.contains(&"sea".to_string()));
}

#[test]
fn test_add_interpretation() {
    let mut symbol = Symbol::new(
        "snake".to_string(),
        "Snake".to_string(),
        "animal".to_string(),
        "Reptile with no limbs".to_string(),
    );

    symbol.add_interpretation(
        "jungian".to_string(),
        "Symbol of transformation and rebirth".to_string(),
    );
    symbol.add_interpretation(
        "mythological".to_string(),
        "Often associated with wisdom and healing".to_string(),
    );

    assert_eq!(symbol.interpretations.len(), 2);
    assert_eq!(
        symbol.interpretations.get("jungian").unwrap(),
        "Symbol of transformation and rebirth"
    );
    assert_eq!(
        symbol.interpretations.get("mythological").unwrap(),
        "Often associated with wisdom and healing"
    );
}

#[test]
fn test_add_related_symbol() {
    let mut symbol = Symbol::new(
        "moon".to_string(),
        "Moon".to_string(),
        "celestial".to_string(),
        "Earth's natural satellite".to_string(),
    );

    symbol.add_related_symbol("night".to_string());
    symbol.add_related_symbol("tides".to_string());

    assert_eq!(symbol.related_symbols.len(), 2);
    assert!(symbol.related_symbols.contains(&"night".to_string()));
    assert!(symbol.related_symbols.contains(&"tides".to_string()));
}
