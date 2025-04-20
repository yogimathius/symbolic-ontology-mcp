use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use dream_ontology_mcp::domain::{RepositoryFactory, Symbol};
use dream_ontology_mcp::infrastructure::postgres_repository::PostgresRepositoryFactory;

// Import the Symbol struct for deserialization
#[derive(Debug, Deserialize, Serialize)]
struct SymbolData {
    id: String,
    name: String,
    category: String,
    description: String,
    interpretations: std::collections::HashMap<String, String>,
    related_symbols: Vec<String>,
    properties: std::collections::HashMap<String, String>,
}

/// Read symbols from a JSON file
fn read_json_file<P: AsRef<Path>>(path: P) -> Result<Vec<SymbolData>, Box<dyn Error>> {
    println!("Reading JSON file: {}", path.as_ref().display());
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let symbols: Vec<SymbolData> = serde_json::from_reader(reader)?;
    println!("Read {} symbols from JSON file", symbols.len());
    Ok(symbols)
}

/// Convert SymbolData to Symbol domain model
fn symbol_data_to_symbol(data: SymbolData) -> Symbol {
    let mut symbol = Symbol::new(data.id, data.name, data.category, data.description);

    // Add interpretations
    symbol.interpretations = data.interpretations;

    // Add related symbols
    symbol.related_symbols = data.related_symbols;

    // Add properties
    symbol.properties = data.properties;

    symbol
}

/// Process the file and seed the database
async fn process_file_and_seed(database_url: &str, json_path: &Path) -> Result<(), Box<dyn Error>> {
    println!("Connecting to database at {}", database_url);

    // Connect to the database
    let factory = PostgresRepositoryFactory::new(database_url).await?;
    let symbol_repo = factory.create_symbol_repository();

    // Read symbols from JSON file
    let symbols_data = read_json_file(json_path)?;
    println!(
        "Successfully parsed {} mythological symbols",
        symbols_data.len()
    );

    // Insert symbols into the database
    let mut success_count = 0;
    let mut error_count = 0;

    for symbol_data in symbols_data {
        let symbol = symbol_data_to_symbol(symbol_data);

        println!("Adding symbol: {}", symbol.name);
        match symbol_repo.create_symbol(symbol).await {
            Ok(_) => {
                success_count += 1;
                println!("Successfully added symbol: {}", success_count);
            }
            Err(err) => {
                error_count += 1;
                eprintln!(
                    "Error creating symbol {}: {}",
                    success_count + error_count,
                    err
                );
            }
        }
    }

    println!("Mythological symbols seeding complete!");
    println!("Successfully imported {} symbols", success_count);
    println!("Errors encountered: {}", error_count);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get database URL from environment variable or use default
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@localhost:5432/symbol_ontology".to_string()
    });

    // Parse command line arguments for JSON file path
    let args: Vec<String> = std::env::args().collect();

    let json_path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        PathBuf::from("data/myth-symbol-seed.json")
    };

    if !json_path.exists() {
        eprintln!("Error: File not found: {}", json_path.display());
        eprintln!("Please provide a valid path to a JSON file containing mythological symbols.");
        eprintln!("Usage: cargo run --bin myth_seeder [path/to/myth-symbols.json]");
        return Err("File not found".into());
    }

    println!("Starting mythological symbols seeder...");
    process_file_and_seed(&database_url, &json_path).await?;

    Ok(())
}
