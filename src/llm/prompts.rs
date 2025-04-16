use crate::domain::Symbol;

/// Template for the system prompt when interpreting symbols
pub const SYSTEM_PROMPT: &str = r#"
You are an expert interpreter of symbolic content, specializing in {category} symbolism.
You provide insightful, nuanced interpretations based on established psychological and cultural frameworks.
Respond with clear, thoughtful analysis without unnecessary disclaimers.
Base your interpretations on established symbolic meanings and psychological principles.
"#;

/// Builds a system prompt for a specific symbol category
pub fn build_system_prompt(category: &str) -> String {
    SYSTEM_PROMPT.replace("{category}", category)
}

/// Template for the user prompt when interpreting a specific symbol
pub const SYMBOL_PROMPT: &str = r#"
Interpret the symbol "{symbol_name}" which is described as: {symbol_description}

{context_instruction}

{specific_query}

Provide a structured interpretation that includes:
1. Core symbolic meaning
2. Psychological significance
3. Cultural and historical context
4. Practical implications
"#;

/// Builds a user prompt for interpreting a symbol
pub fn build_symbol_prompt(symbol: &Symbol, context: Option<&str>, query: Option<&str>) -> String {
    let context_instruction = match context {
        Some(ctx) => format!("Consider the specific context: {}", ctx),
        None => "Consider general interpretive contexts".to_string(),
    };

    let specific_query = match query {
        Some(q) => format!("Specifically address this question: {}", q),
        None => "Provide a general interpretation".to_string(),
    };

    SYMBOL_PROMPT
        .replace("{symbol_name}", &symbol.name)
        .replace("{symbol_description}", &symbol.description)
        .replace("{context_instruction}", &context_instruction)
        .replace("{specific_query}", &specific_query)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_symbol() -> Symbol {
        Symbol {
            id: "water".to_string(),
            name: "Water".to_string(),
            category: "dream".to_string(),
            description: "Symbolizes emotions and the unconscious".to_string(),
            interpretations: HashMap::new(),
            related_symbols: vec!["ocean".to_string()],
            properties: HashMap::new(),
        }
    }

    #[test]
    fn test_build_system_prompt() {
        let prompt = build_system_prompt("dream");
        assert!(prompt.contains("dream symbolism"));
    }

    #[test]
    fn test_build_symbol_prompt() {
        let symbol = create_test_symbol();

        // Test with no context or query
        let basic_prompt = build_symbol_prompt(&symbol, None, None);
        assert!(basic_prompt.contains("Water"));
        assert!(basic_prompt.contains("Symbolizes emotions and the unconscious"));
        assert!(basic_prompt.contains("Consider general interpretive contexts"));
        assert!(basic_prompt.contains("Provide a general interpretation"));

        // Test with context and query
        let detailed_prompt = build_symbol_prompt(
            &symbol,
            Some("recurring nightmares"),
            Some("Why does this symbol appear during times of stress?"),
        );

        assert!(detailed_prompt.contains("Consider the specific context: recurring nightmares"));
        assert!(detailed_prompt.contains("Specifically address this question: Why does this symbol appear during times of stress?"));
    }
}
