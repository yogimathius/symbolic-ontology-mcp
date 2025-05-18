use crate::common::fixtures;
use dream_ontology_mcp::llm::prompts::{build_symbol_prompt, build_system_prompt};

#[test]
fn test_build_system_prompt() {
    let prompt = build_system_prompt("dream");
    assert!(prompt.contains("dream symbolism"));

    let prompt = build_system_prompt("mythological");
    assert!(prompt.contains("mythological symbolism"));
}

#[test]
fn test_build_symbol_prompt_basic() {
    let symbol = fixtures::create_test_symbol("water", "Water", "dream");

    let basic_prompt = build_symbol_prompt(&symbol, None, None);
    assert!(basic_prompt.contains("Water"));
    assert!(basic_prompt.contains("Description for Water"));
    assert!(basic_prompt.contains("Consider general interpretive contexts"));
    assert!(basic_prompt.contains("Provide a general interpretation"));
}

#[test]
fn test_build_symbol_prompt_detailed() {
    let symbol = fixtures::create_test_symbol("water", "Water", "dream");

    let detailed_prompt = build_symbol_prompt(
        &symbol,
        Some("recurring nightmares"),
        Some("Why does this symbol appear during times of stress?"),
    );

    assert!(detailed_prompt.contains("Consider the specific context: recurring nightmares"));
    assert!(detailed_prompt.contains(
        "Specifically address this question: Why does this symbol appear during times of stress?"
    ));
}

#[test]
fn test_build_symbol_prompt_structure() {
    let symbol = fixtures::create_detailed_symbol(
        "fire",
        "Fire",
        "dream",
        "Symbol of transformation and passion",
        vec![
            ("psychology", "Represents energy, desire or anger"),
            ("spiritual", "Symbol of enlightenment and purification"),
        ],
        vec!["light", "sun"],
    );

    let prompt = build_symbol_prompt(&symbol, None, None);

    assert!(prompt.contains("Core symbolic meaning"));
    assert!(prompt.contains("Psychological significance"));
    assert!(prompt.contains("Cultural and historical context"));
    assert!(prompt.contains("Practical implications"));
}
