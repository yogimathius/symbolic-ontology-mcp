use rmcp::schemars;
use serde::{Deserialize, Serialize};

/// Parameters for the get_symbols MCP method
///
/// This schema follows the Model Context Protocol (MCP) specification for method parameters.
/// Reference: https://modelcontextprotocol.io
///
/// The `get_symbols` method allows clients to query the symbolic ontology without filtering.
/// For category filtering, use filter_by_category. For text search, use search_symbols.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetSymbolsParams {
    /// Maximum number of symbols to return
    #[serde(default = "default_limit")]
    pub limit: usize,
}

/// Parameters for the search_symbols MCP method (with non-optional query parameter)
///
/// This is a workaround for Cursor MCP client issues with Option<String> parameters.
/// It provides a direct search endpoint with a required query parameter.
/// RECOMMENDED: Use this method for all text searches.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SearchSymbolsParams {
    /// Search query for symbol names or descriptions (required)
    pub query: String,

    /// Maximum number of symbols to return
    #[serde(default = "default_limit")]
    pub limit: usize,
}

/// Parameters for the filter_by_category MCP method (with non-optional category parameter)
///
/// This is a workaround for Cursor MCP client issues with Option<String> parameters.
/// It provides a direct category filtering endpoint with a required category parameter.
/// RECOMMENDED: Use this method for all category filtering.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct CategorySymbolsParams {
    /// Category filter (dream, mythological, etc.) - required
    pub category: String,

    /// Maximum number of symbols to return
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    50
}

/// Response for the get_symbols, search_symbols, and filter_by_category MCP methods
///
/// This schema follows the Model Context Protocol (MCP) specification for method responses.
/// Reference: https://modelcontextprotocol.io
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetSymbolsResponse {
    /// List of symbols matching the query
    pub symbols: Vec<SymbolDTO>,

    /// Total count of symbols matching the query (for pagination)
    pub total_count: usize,
}

/// Response for the get_categories MCP method
///
/// Returns all available categories in the symbol ontology
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetCategoriesResponse {
    /// List of available categories
    pub categories: Vec<String>,

    /// Total number of categories
    pub count: usize,
}

/// Data transfer object for Symbol, used in MCP responses
///
/// This DTO represents the Symbol domain model in a serializable format
/// suitable for JSON transmission via the MCP protocol.
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{self};

    #[test]
    fn test_get_symbols_params_serialization() {
        let params = GetSymbolsParams { limit: 10 };

        let json = serde_json::to_value(params).unwrap();

        assert_eq!(json["limit"], 10);
    }

    #[test]
    fn test_get_symbols_params_default_limit() {
        let params = GetSymbolsParams {
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

    #[test]
    fn test_get_categories_response_serialization() {
        let response = GetCategoriesResponse {
            categories: vec!["nature".to_string(), "jungian".to_string()],
            count: 2,
        };

        let json = serde_json::to_value(response).unwrap();

        assert_eq!(json["categories"][0], "nature");
        assert_eq!(json["categories"][1], "jungian");
        assert_eq!(json["count"], 2);
    }
}
