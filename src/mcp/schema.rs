use rmcp::schemars;
use serde::{Deserialize, Serialize};

/// Parameters for the get_symbols MCP method
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetSymbolsParams {
    /// Category filter (dream, mythological, etc.)
    pub category: Option<String>,

    /// Search query for symbol names or descriptions
    pub query: Option<String>,

    /// Maximum number of symbols to return
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    50
}

/// Response for the get_symbols MCP method
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetSymbolsResponse {
    /// List of symbols matching the query
    pub symbols: Vec<SymbolDTO>,

    /// Total count of symbols matching the query (for pagination)
    pub total_count: usize,
}

/// Data transfer object for Symbol, used in MCP responses
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SymbolDTO {
    /// Unique identifier for the symbol
    pub id: String,

    /// Name of the symbol
    pub name: String,

    /// Category of the symbol
    pub category: String,

    /// Brief description of the symbol
    pub description: String,

    /// Related symbol IDs
    pub related_symbols: Vec<String>,
}

/// Parameters for the interpret_symbol MCP method
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct InterpretSymbolParams {
    /// The symbol ID to interpret
    pub symbol_id: String,

    /// Optional context for the interpretation
    pub context: Option<String>,

    /// Optional specific question about the symbol
    pub query: Option<String>,
}

/// Response for the interpret_symbol MCP method
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct InterpretSymbolResponse {
    /// The interpreted symbol ID
    pub symbol_id: String,

    /// The interpretation text
    pub interpretation: String,

    /// The context that was used for interpretation
    pub context: Option<String>,

    /// Related symbols mentioned in the interpretation
    pub related_symbols: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{self, json};

    #[test]
    fn test_get_symbols_params_serialization() {
        let params = GetSymbolsParams {
            category: Some("dream".to_string()),
            query: Some("water".to_string()),
            limit: 10,
        };

        let json = serde_json::to_value(params).unwrap();

        assert_eq!(json["category"], "dream");
        assert_eq!(json["query"], "water");
        assert_eq!(json["limit"], 10);
    }

    #[test]
    fn test_get_symbols_params_default_limit() {
        let params = GetSymbolsParams {
            category: None,
            query: None,
            limit: default_limit(),
        };

        assert_eq!(params.limit, 50);
    }

    #[test]
    fn test_symbol_dto_serialization() {
        let symbol = SymbolDTO {
            id: "water".to_string(),
            name: "Water".to_string(),
            category: "dream".to_string(),
            description: "Symbolizes emotions".to_string(),
            related_symbols: vec!["ocean".to_string(), "river".to_string()],
        };

        let json = serde_json::to_value(symbol).unwrap();

        assert_eq!(json["id"], "water");
        assert_eq!(json["name"], "Water");
        assert_eq!(json["category"], "dream");
        assert_eq!(json["description"], "Symbolizes emotions");
        assert_eq!(json["related_symbols"][0], "ocean");
        assert_eq!(json["related_symbols"][1], "river");
    }
}
