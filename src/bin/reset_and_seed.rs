use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use serde::Deserialize;
use tokio;
use csv::ReaderBuilder;

use dream_ontology_mcp::domain::{RepositoryFactory, Symbol, SymbolRepository};
use dream_ontology_mcp::infrastructure::postgres_repository::PostgresRepositoryFactory;

// Import the Symbol structs for deserialization (same as in download_and_seed.rs)
#[derive(Debug, Deserialize)]
struct DreamDictionaryRow {
    #[serde(rename = "Symbol")]
    symbol: Option<String>,
    #[serde(rename = "Interpretation")]
    interpretation: Option<String>,
    #[serde(rename = "Description", default)]
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DictionaryOfDreamsRow {
    #[serde(rename = "Symbol")]
    symbol: Option<String>,
    #[serde(rename = "Meaning")]
    meaning: Option<String>,
    #[serde(rename = "Description", default)]
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DreamsInterpretationsRow {
    #[serde(rename = "Dream Symbol")]
    symbol: Option<String>,
    #[serde(rename = "Interpretation")]
    interpretation: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CleanedDreamInterpretationsRow {
    #[serde(rename = "Word")]
    word: Option<String>,
    #[serde(rename = "Interpretation")]
    interpretation: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SampleDreamSymbolRow {
    #[serde(rename = "Symbol")]
    symbol: Option<String>,
    #[serde(rename = "Interpretation")]
    interpretation: Option<String>,
    #[serde(rename = "Description")]
    description: Option<String>,
}

// Function for myth symbol JSON deserialization
#[derive(Debug, Deserialize)]
struct SymbolData {
    id: String,
    name: String,
    category: String,
    description: String,
    interpretations: HashMap<String, String>,
    related_symbols: Vec<String>,
    properties: HashMap<String, String>,
}

// Generic reader function to handle different CSV formats
fn read_csv_file<P: AsRef<Path>, T: for<'de> Deserialize<'de>>(
    path: P,
) -> Result<Vec<T>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut csv_reader = ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(reader);

    let records: Vec<T> = csv_reader
        .deserialize()
        .filter_map(|result| match result {
            Ok(record) => Some(record),
            Err(err) => {
                eprintln!("Error parsing CSV row: {}", err);
                None
            }
        })
        .collect();

    Ok(records)
}

// Function to convert a row to a Symbol
fn row_to_symbol(symbol_name: &str, interpretation: &str, description: Option<&str>) -> Symbol {
    let id = symbol_name.to_lowercase().replace(" ", "_");
    let name = symbol_name.to_string();
    let category = "dream".to_string();
    let description = description
        .map(|d| d.to_string())
        .unwrap_or_else(|| "No description".to_string());

    let mut interpretations = HashMap::new();
    interpretations.insert("default".to_string(), interpretation.to_string());

    let mut symbol = Symbol::new(id, name, category, description);
    symbol.interpretations = interpretations;
    symbol
}

// Function to read the mythological symbols from JSON
fn read_myth_symbols(json_path: &Path) -> Result<Vec<Symbol>, Box<dyn Error>> {
    println!("Reading mythology JSON file: {}", json_path.display());
    
    if !json_path.exists() {
        return Err(format!("Myth symbols JSON file not found: {}", json_path.display()).into());
    }
    
    let file = File::open(json_path)?;
    let reader = BufReader::new(file);
    
    let symbols_data: Vec<SymbolData> = serde_json::from_reader(reader)?;
    println!("Read {} mythology symbols from JSON file", symbols_data.len());
    
    // Convert to domain Symbol objects
    let symbols = symbols_data
        .into_iter()
        .map(|data| {
            let mut symbol = Symbol::new(
                data.id,
                data.name,
                data.category,
                data.description
            );
            
            symbol.interpretations = data.interpretations;
            symbol.related_symbols = data.related_symbols;
            symbol.properties = data.properties;
            
            symbol
        })
        .collect();
    
    Ok(symbols)
}

// Reset the database by deleting all symbols
async fn reset_database(symbol_repo: Arc<dyn SymbolRepository>) -> Result<(), Box<dyn Error>> {
    println!("Resetting database - clearing all symbols...");
    
    // Get all symbols
    let symbols = symbol_repo.list_symbols(None).await?;
    println!("Found {} symbols to delete", symbols.len());
    
    let mut deleted_count = 0;
    
    // Delete each symbol
    for symbol in symbols {
        match symbol_repo.delete_symbol(&symbol.id).await {
            Ok(_) => {
                deleted_count += 1;
                if deleted_count % 100 == 0 {
                    println!("Deleted {} symbols...", deleted_count);
                }
            },
            Err(err) => {
                eprintln!("Error deleting symbol {}: {}", symbol.id, err);
            }
        }
    }
    
    println!("Database reset complete. Deleted {} symbols.", deleted_count);
    Ok(())
}

// Main processing function to seed the database
async fn process_and_seed(
    database_url: &str,
    data_dir: &Path,
) -> Result<(), Box<dyn Error>> {
    println!("Connecting to database at {}", database_url);

    // Connect to the database
    let factory = PostgresRepositoryFactory::new(database_url).await?;
    let symbol_repo = factory.create_symbol_repository();
    
    // Reset the database first
    reset_database(symbol_repo.clone()).await?;

    // Track success and error counts
    let mut success_count = 0;
    let mut error_count = 0;
    
    // Process mythology symbols first
    let myth_path = data_dir.join("myth-symbol-seed.json");
    if myth_path.exists() {
        let myth_symbols = read_myth_symbols(&myth_path)?;
        println!("Importing {} mythology symbols...", myth_symbols.len());
        
        for symbol in myth_symbols {
            println!("Adding mythology symbol: {}", symbol.name);
            match symbol_repo.create_symbol(symbol).await {
                Ok(_) => {
                    success_count += 1;
                    println!("Successfully added mythology symbol #{}", success_count);
                },
                Err(err) => {
                    error_count += 1;
                    eprintln!("Error creating mythology symbol: {}", err);
                }
            }
        }
    }

    // Find all CSV files in the data directory
    let mut csv_files = Vec::new();
    for entry in fs::read_dir(data_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "csv") {
            csv_files.push(path);
        }
    }

    // Process each CSV file
    for path in csv_files {
        let path_str = path.to_string_lossy();
        println!("Processing file: {}", path_str);

        if path_str.contains("dream-dictionary") || path_str.contains("Dream Dictionary") {
            let records: Vec<DreamDictionaryRow> = match read_csv_file(&path) {
                Ok(records) => records,
                Err(err) => {
                    eprintln!("Error reading CSV file {}: {}", path_str, err);
                    continue;
                }
            };

            println!("Importing {} dream dictionary symbols...", records.len());
            
            for record in records {
                if let (Some(symbol_name), Some(interpretation)) =
                    (record.symbol, record.interpretation)
                {
                    if symbol_name.trim().is_empty() || interpretation.trim().is_empty() {
                        continue;
                    }

                    let symbol =
                        row_to_symbol(&symbol_name, &interpretation, record.description.as_deref());

                    match symbol_repo.create_symbol(symbol).await {
                        Ok(_) => {
                            success_count += 1;
                            if success_count % 100 == 0 {
                                println!("Processed {} symbols...", success_count);
                            }
                        }
                        Err(err) => {
                            eprintln!("Error creating symbol {}: {}", symbol_name, err);
                            error_count += 1;
                        }
                    }
                }
            }
        } else if path_str.contains("dictionary-of-dreams")
            || path_str.contains("Dictionary of Dreams")
        {
            let records: Vec<DictionaryOfDreamsRow> = match read_csv_file(&path) {
                Ok(records) => records,
                Err(err) => {
                    eprintln!("Error reading CSV file {}: {}", path_str, err);
                    continue;
                }
            };

            println!("Importing {} dictionary of dreams symbols...", records.len());
            
            for record in records {
                if let (Some(symbol_name), Some(meaning)) = (record.symbol, record.meaning) {
                    if symbol_name.trim().is_empty() || meaning.trim().is_empty() {
                        continue;
                    }

                    let symbol =
                        row_to_symbol(&symbol_name, &meaning, record.description.as_deref());

                    match symbol_repo.create_symbol(symbol).await {
                        Ok(_) => {
                            success_count += 1;
                            if success_count % 100 == 0 {
                                println!("Processed {} symbols...", success_count);
                            }
                        }
                        Err(err) => {
                            eprintln!("Error creating symbol {}: {}", symbol_name, err);
                            error_count += 1;
                        }
                    }
                }
            }
        } else if path_str.contains("dreams_interpretations") {
            let records: Vec<DreamsInterpretationsRow> = match read_csv_file(&path) {
                Ok(records) => records,
                Err(err) => {
                    eprintln!("Error reading CSV file {}: {}", path_str, err);
                    continue;
                }
            };

            println!("Importing {} dreams interpretations symbols...", records.len());
            
            for record in records {
                if let (Some(symbol_name), Some(interpretation)) =
                    (record.symbol, record.interpretation)
                {
                    if symbol_name.trim().is_empty() || interpretation.trim().is_empty() {
                        continue;
                    }

                    let symbol = row_to_symbol(&symbol_name, &interpretation, None);

                    match symbol_repo.create_symbol(symbol).await {
                        Ok(_) => {
                            success_count += 1;
                            if success_count % 100 == 0 {
                                println!("Processed {} symbols...", success_count);
                            }
                        }
                        Err(err) => {
                            eprintln!("Error creating symbol {}: {}", symbol_name, err);
                            error_count += 1;
                        }
                    }
                }
            }
        } else if path_str.contains("cleaned_dream_interpretations")
            || path_str.contains("dream_interpretations_words")
        {
            let records: Vec<CleanedDreamInterpretationsRow> = match read_csv_file(&path) {
                Ok(records) => records,
                Err(err) => {
                    eprintln!("Error reading CSV file {}: {}", path_str, err);
                    continue;
                }
            };

            println!("Importing {} cleaned dream interpretations symbols...", records.len());
            
            for record in records {
                if let (Some(word), Some(interpretation)) = (record.word, record.interpretation) {
                    if word.trim().is_empty() || interpretation.trim().is_empty() {
                        continue;
                    }

                    let symbol = row_to_symbol(&word, &interpretation, None);

                    match symbol_repo.create_symbol(symbol).await {
                        Ok(_) => {
                            success_count += 1;
                            if success_count % 100 == 0 {
                                println!("Processed {} symbols...", success_count);
                            }
                        }
                        Err(err) => {
                            eprintln!("Error creating symbol {}: {}", word, err);
                            error_count += 1;
                        }
                    }
                }
            }
        } else if path_str.contains("sample_dream_symbols") {
            let records: Vec<SampleDreamSymbolRow> = match read_csv_file(&path) {
                Ok(records) => records,
                Err(err) => {
                    eprintln!("Error reading CSV file {}: {}", path_str, err);
                    continue;
                }
            };

            println!("Importing {} sample dream symbols...", records.len());
            
            for record in records {
                if let (Some(symbol_name), Some(interpretation)) =
                    (record.symbol, record.interpretation)
                {
                    if symbol_name.trim().is_empty() || interpretation.trim().is_empty() {
                        continue;
                    }

                    let symbol =
                        row_to_symbol(&symbol_name, &interpretation, record.description.as_deref());

                    match symbol_repo.create_symbol(symbol).await {
                        Ok(_) => {
                            success_count += 1;
                            if success_count % 100 == 0 {
                                println!("Processed {} symbols...", success_count);
                            }
                        }
                        Err(err) => {
                            eprintln!("Error creating symbol {}: {}", symbol_name, err);
                            error_count += 1;
                        }
                    }
                }
            }
        } else {
            eprintln!("Unknown file format: {}", path_str);
        }
    }

    println!("Database seeding complete!");
    println!("Successfully imported {} symbols", success_count);
    println!("Errors encountered: {}", error_count);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get database URL from environment variable or use default
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/symbol_ontology".to_string());

    // Create data directory
    let data_dir = PathBuf::from("data");
    if !data_dir.exists() {
        eprintln!("Error: Data directory not found: {}", data_dir.display());
        eprintln!("Please create the data directory with CSV and JSON files.");
        return Err("Data directory not found".into());
    }
    
    println!("Starting database reset and seed process...");
    process_and_seed(&database_url, &data_dir).await?;

    println!("All processing complete!");
    Ok(())
} 