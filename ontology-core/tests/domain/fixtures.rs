use ontology_core::domain::{Symbol, SymbolSet};

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

    symbol_set.add_symbol(create_test_symbol("water", "Water", "nature"));
    symbol_set.add_symbol(create_test_symbol("fire", "Fire", "nature"));
    symbol_set.add_symbol(create_test_symbol("mountain", "Mountain", "nature"));

    symbol_set
}
