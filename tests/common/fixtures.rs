use dream_ontology_mcp::domain::{Symbol, SymbolSet};

pub fn create_test_symbol(id: &str, name: &str, category: &str) -> Symbol {
    Symbol::new(
        id.to_string(),
        name.to_string(),
        category.to_string(),
        format!("Description for {}", name),
    )
}

pub fn create_detailed_symbol(
    id: &str,
    name: &str,
    category: &str,
    description: &str,
    interpretations: Vec<(&str, &str)>,
    related_symbols: Vec<&str>,
) -> Symbol {
    let mut symbol = Symbol::new(
        id.to_string(),
        name.to_string(),
        category.to_string(),
        description.to_string(),
    );

    for (context, interpretation) in interpretations {
        symbol.add_interpretation(context.to_string(), interpretation.to_string());
    }

    for related in related_symbols {
        symbol.add_related_symbol(related.to_string());
    }

    symbol
}

pub fn create_test_symbol_set(
    id: &str,
    name: &str,
    category: &str,
    description: &str,
) -> SymbolSet {
    let mut symbol_set = SymbolSet::new(
        id.to_string(),
        name.to_string(),
        category.to_string(),
        description.to_string(),
    );

    symbol_set.add_symbol(create_test_symbol("water", "Water", "dream"));
    symbol_set.add_symbol(create_test_symbol("fire", "Fire", "dream"));
    symbol_set.add_symbol(create_test_symbol("mountain", "Mountain", "dream"));

    symbol_set
}

pub fn create_dream_symbols() -> SymbolSet {
    let mut symbol_set = SymbolSet::new(
        "dream-symbols".to_string(),
        "Dream Symbols".to_string(),
        "dream".to_string(),
        "Common symbols appearing in dreams".to_string(),
    );

    let water = create_detailed_symbol(
        "water",
        "Water",
        "dream",
        "Symbolizes emotions and the unconscious",
        vec![
            (
                "psychology",
                "Represents the depths of the unconscious mind",
            ),
            ("spiritual", "Symbol of purification and renewal"),
        ],
        vec!["ocean", "river"],
    );

    let fire = create_detailed_symbol(
        "fire",
        "Fire",
        "dream",
        "Symbolizes transformation and passion",
        vec![
            ("psychology", "Represents energy, desire or anger"),
            ("spiritual", "Symbol of enlightenment and purification"),
        ],
        vec!["light", "sun"],
    );

    symbol_set.add_symbol(water);
    symbol_set.add_symbol(fire);

    symbol_set
}
