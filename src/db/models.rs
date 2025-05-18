use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, postgres::PgRow};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub id: String,

    pub name: String,

    pub category: String,

    pub description: String,

    pub interpretations: HashMap<String, String>,

    pub related_symbols: Vec<String>,

    pub properties: HashMap<String, String>,
}

impl FromRow<'_, PgRow> for Symbol {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let category: String = row.try_get("category")?;
        let description: String = row.try_get("description")?;

        let interpretations_json: serde_json::Value = row
            .try_get("interpretations")
            .unwrap_or_else(|_| serde_json::json!({}));

        let related_symbols_json: serde_json::Value = row
            .try_get("related_symbols")
            .unwrap_or_else(|_| serde_json::json!([]));

        let properties_json: serde_json::Value = row
            .try_get("properties")
            .unwrap_or_else(|_| serde_json::json!({}));

        let interpretations =
            serde_json::from_value(interpretations_json).unwrap_or_else(|_| HashMap::new());

        let related_symbols =
            serde_json::from_value(related_symbols_json).unwrap_or_else(|_| Vec::new());

        let properties = serde_json::from_value(properties_json).unwrap_or_else(|_| HashMap::new());

        Ok(Symbol {
            id,
            name,
            category,
            description,
            interpretations,
            related_symbols,
            properties,
        })
    }
}

impl Symbol {
    pub fn new(id: String, name: String, category: String, description: String) -> Self {
        Symbol {
            id,
            name,
            category,
            description,
            interpretations: HashMap::new(),
            related_symbols: Vec::new(),
            properties: HashMap::new(),
        }
    }

    pub fn with_category(mut self, category: &str) -> Self {
        self.category = category.to_string();
        self
    }

    pub fn with_related(mut self, related: Vec<&str>) -> Self {
        self.related_symbols = related.into_iter().map(|s| s.to_string()).collect();
        self
    }

    #[allow(dead_code)]
    pub fn add_related_symbol(&mut self, symbol_id: String) {
        self.related_symbols.push(symbol_id);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolSet {
    pub id: String,

    pub name: String,

    pub category: String,

    pub description: String,

    pub symbols_map: HashMap<String, serde_json::Value>,
}

impl FromRow<'_, PgRow> for SymbolSet {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let category: String = row.try_get("category")?;
        let description: String = row.try_get("description")?;

        let symbols_json: serde_json::Value = row
            .try_get("symbols")
            .unwrap_or_else(|_| serde_json::json!({}));

        let symbols_map = serde_json::from_value(symbols_json).unwrap_or_else(|_| HashMap::new());

        Ok(SymbolSet {
            id,
            name,
            category,
            description,
            symbols_map,
        })
    }
}

impl SymbolSet {
    pub fn new(id: String, name: String, category: String, description: String) -> Self {
        SymbolSet {
            id,
            name,
            category,
            description,
            symbols_map: HashMap::new(),
        }
    }

    pub fn with_symbols(mut self, symbol_ids: Vec<&str>) -> Self {
        for id in symbol_ids {
            self.symbols_map
                .insert(id.to_string(), serde_json::Value::Null);
        }
        self
    }

    pub fn from_domain(set: crate::domain::SymbolSet) -> Self {
        let mut symbols_map = HashMap::new();

        for (id, _) in set.symbols {
            symbols_map.insert(id, serde_json::Value::Null);
        }

        Self {
            id: set.id,
            name: set.name,
            category: set.category,
            description: set.description,
            symbols_map,
        }
    }

    pub fn to_domain(&self, symbols: &[Symbol]) -> crate::domain::SymbolSet {
        let mut result = crate::domain::SymbolSet::new(
            self.id.clone(),
            self.name.clone(),
            self.category.clone(),
            self.description.clone(),
        );

        for symbol in symbols {
            if self.symbols_map.contains_key(&symbol.id) {
                let domain_symbol = crate::domain::Symbol {
                    id: symbol.id.clone(),
                    name: symbol.name.clone(),
                    category: symbol.category.clone(),
                    description: symbol.description.clone(),
                    interpretations: symbol.interpretations.clone(),
                    related_symbols: symbol.related_symbols.clone(),
                    properties: symbol.properties.clone(),
                };
                result.add_symbol(domain_symbol);
            }
        }

        result
    }
}
