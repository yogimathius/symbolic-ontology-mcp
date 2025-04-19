use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

// Import domain and repository types
use dream_ontology_mcp::domain::{RepositoryFactory, Symbol, SymbolRepository};
use dream_ontology_mcp::infrastructure::postgres_repository::PostgresRepositoryFactory;

#[derive(Debug, Deserialize)]
struct DreamDictionaryRow {
    #[serde(rename = "Symbol")]
    symbol: Option<String>,
    #[serde(rename = "Interpretation")]
    interpretation: Option<String>,
    // Add other possible field names
    #[serde(rename = "Description", default)]
    description: Option<String>,
    #[serde(rename = "Category", default)]
    category: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DictionaryOfDreamsRow {
    #[serde(rename = "Symbol")]
    symbol: Option<String>,
    #[serde(rename = "Meaning")]
    meaning: Option<String>,
    // Add other possible field names
    #[serde(rename = "Description", default)]
    description: Option<String>,
    #[serde(rename = "Category", default)]
    category: Option<String>,
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

async fn process_files_and_seed(
    database_url: &str,
    csv_files: Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    println!("Connecting to database at {}", database_url);

    // Connect to the database
    let factory = PostgresRepositoryFactory::new(database_url).await?;
    let symbol_repo = factory.create_symbol_repository();

    // Track processed symbols to avoid duplicates
    let mut processed_symbols = HashSet::new();
    let mut success_count = 0;
    let mut error_count = 0;

    // Process each CSV file
    for path in csv_files {
        let path_str = path.to_string_lossy().to_string();
        println!("Processing file: {}", path_str);

        if path_str.contains("dream-dictionary") || path_str.contains("Dream Dictionary") {
            let records: Vec<DreamDictionaryRow> = match read_csv_file(&path) {
                Ok(records) => records,
                Err(err) => {
                    eprintln!("Error reading CSV file {}: {}", path_str, err);
                    continue;
                }
            };

            println!("Found {} records in {}", records.len(), path_str);

            for record in records {
                if let (Some(symbol_name), Some(interpretation)) =
                    (record.symbol, record.interpretation)
                {
                    if symbol_name.trim().is_empty() || interpretation.trim().is_empty() {
                        continue;
                    }

                    let symbol_key = symbol_name.to_lowercase();
                    if processed_symbols.contains(&symbol_key) {
                        continue;
                    }

                    let symbol =
                        row_to_symbol(&symbol_name, &interpretation, record.description.as_deref());

                    match symbol_repo.create_symbol(symbol).await {
                        Ok(_) => {
                            processed_symbols.insert(symbol_key);
                            success_count += 1;
                            if success_count % 10 == 0 {
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

            println!("Found {} records in {}", records.len(), path_str);

            for record in records {
                if let (Some(symbol_name), Some(meaning)) = (record.symbol, record.meaning) {
                    if symbol_name.trim().is_empty() || meaning.trim().is_empty() {
                        continue;
                    }

                    let symbol_key = symbol_name.to_lowercase();
                    if processed_symbols.contains(&symbol_key) {
                        continue;
                    }

                    let symbol =
                        row_to_symbol(&symbol_name, &meaning, record.description.as_deref());

                    match symbol_repo.create_symbol(symbol).await {
                        Ok(_) => {
                            processed_symbols.insert(symbol_key);
                            success_count += 1;
                            if success_count % 10 == 0 {
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
            // Try to guess the format based on the headers
            println!(
                "Unknown file format: {}, attempting to detect format...",
                path_str
            );

            // First try as DreamDictionaryRow
            let dream_dict_result: Result<Vec<DreamDictionaryRow>, _> = read_csv_file(&path);
            if let Ok(records) = dream_dict_result {
                println!(
                    "Detected Dream Dictionary format with {} records",
                    records.len()
                );

                for record in records {
                    if let (Some(symbol_name), Some(interpretation)) =
                        (record.symbol, record.interpretation)
                    {
                        if symbol_name.trim().is_empty() || interpretation.trim().is_empty() {
                            continue;
                        }

                        let symbol_key = symbol_name.to_lowercase();
                        if processed_symbols.contains(&symbol_key) {
                            continue;
                        }

                        let symbol = row_to_symbol(
                            &symbol_name,
                            &interpretation,
                            record.description.as_deref(),
                        );

                        match symbol_repo.create_symbol(symbol).await {
                            Ok(_) => {
                                processed_symbols.insert(symbol_key);
                                success_count += 1;
                                if success_count % 10 == 0 {
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
                // Try as DictionaryOfDreamsRow
                let dict_of_dreams_result: Result<Vec<DictionaryOfDreamsRow>, _> =
                    read_csv_file(&path);
                if let Ok(records) = dict_of_dreams_result {
                    println!(
                        "Detected Dictionary of Dreams format with {} records",
                        records.len()
                    );

                    for record in records {
                        if let (Some(symbol_name), Some(meaning)) = (record.symbol, record.meaning)
                        {
                            if symbol_name.trim().is_empty() || meaning.trim().is_empty() {
                                continue;
                            }

                            let symbol_key = symbol_name.to_lowercase();
                            if processed_symbols.contains(&symbol_key) {
                                continue;
                            }

                            let symbol = row_to_symbol(
                                &symbol_name,
                                &meaning,
                                record.description.as_deref(),
                            );

                            match symbol_repo.create_symbol(symbol).await {
                                Ok(_) => {
                                    processed_symbols.insert(symbol_key);
                                    success_count += 1;
                                    if success_count % 10 == 0 {
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
                    eprintln!("Could not detect CSV format for {}", path_str);
                }
            }
        }
    }

    println!("Database seeding complete!");
    println!("Successfully imported {} symbols", success_count);
    println!("Errors encountered: {}", error_count);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Print usage instructions
    println!("Dream Ontology Database Seeder");
    println!("------------------------------");
    println!("This tool seeds your database with dream symbols from CSV files.");
    println!();
    println!("NOTE: You need to manually download the CSV files from Kaggle:");
    println!("1. Dream Dictionary: https://www.kaggle.com/datasets/yuvrajsanghai/dream-dictionary");
    println!(
        "2. Dictionary of Dreams: https://www.kaggle.com/datasets/manswad/dictionary-of-dreams"
    );
    println!();

    // Get database URL from environment variable or use default
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/symbol_ontology".to_string());

    // Get CSV file paths from command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        println!("ERROR: You must provide at least one CSV file path.");
        println!(
            "Usage: cargo run --bin manual_seed path/to/dream_dictionary.csv [path/to/other.csv ...]"
        );
        return Ok(());
    }

    let csv_paths: Vec<PathBuf> = args[1..].iter().map(PathBuf::from).collect();

    // Check if all files exist
    let mut missing_files = false;
    for path in &csv_paths {
        if !path.exists() {
            eprintln!("Error: File not found: {}", path.display());
            missing_files = true;
        }
    }

    if missing_files {
        eprintln!("Some specified files were not found. Please check the paths and try again.");
        return Ok(());
    }

    // Process files and seed database
    process_files_and_seed(&database_url, csv_paths).await?;

    Ok(())
}
