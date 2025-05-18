use clap::{ArgAction, Parser};
use csv::ReaderBuilder;
use reqwest;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use tokio;
use zip::ZipArchive;

use dream_ontology_mcp::db::models::Symbol;
use dream_ontology_mcp::db::pool::{DbError, create_pool, init_database};
use dream_ontology_mcp::db::queries::SymbolQueries;

#[derive(Parser, Debug)]
#[clap(author, version, about = "Seed the Symbol Ontology database")]
struct Args {
    #[clap(short, long)]
    files: Vec<PathBuf>,

    #[clap(short, long, action=ArgAction::SetTrue)]
    download: bool,

    #[clap(short, long, action=ArgAction::SetTrue)]
    reset: bool,

    #[clap(short, long, action=ArgAction::SetTrue)]
    test_data: bool,

    #[clap(long)]
    database_url: Option<String>,

    #[clap(short, long, default_value = "data")]
    output_dir: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
struct SymbolData {
    id: String,
    name: String,
    category: String,
    description: String,
    interpretations: HashMap<String, String>,
    related_symbols: Vec<String>,
    properties: HashMap<String, String>,
}

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

fn symbol_data_to_symbol(data: SymbolData) -> Symbol {
    let mut symbol = Symbol::new(data.id, data.name, data.category, data.description);
    symbol.interpretations = data.interpretations;
    symbol.related_symbols = data.related_symbols;
    symbol.properties = data.properties;
    symbol
}

async fn import_json_file<P: AsRef<Path>>(
    pool: &PgPool,
    path: P,
) -> Result<(usize, usize), Box<dyn Error>> {
    println!("Reading JSON file: {}", path.as_ref().display());

    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let start = Instant::now();
    let symbols_data: Vec<SymbolData> = serde_json::from_reader(reader)?;

    println!(
        "Read {} symbols from JSON file in {:?}",
        symbols_data.len(),
        start.elapsed()
    );

    let mut success_count = 0;
    let mut error_count = 0;
    let total_symbols = symbols_data.len();

    for symbol_data in symbols_data {
        let symbol = symbol_data_to_symbol(symbol_data);

        if success_count % 100 == 0 {
            println!(
                "Processing symbol: {} ({}/{})",
                symbol.name, success_count, total_symbols
            );
        }

        match SymbolQueries::create(pool, &symbol).await {
            Ok(_) => {
                success_count += 1;
            }
            Err(DbError::Conflict(_)) => {
                error_count += 1;
            }
            Err(err) => {
                eprintln!("Error creating symbol {}: {}", symbol.name, err);
                error_count += 1;
            }
        }
    }

    println!(
        "JSON import complete: {} succeeded, {} errors",
        success_count, error_count
    );

    Ok((success_count, error_count))
}

fn read_csv_file<P: AsRef<Path>, T: for<'de> Deserialize<'de>>(
    path: P,
) -> Result<Vec<T>, Box<dyn Error>> {
    let file = File::open(&path)?;
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

async fn import_csv_file<P: AsRef<Path>>(
    pool: &PgPool,
    path: P,
) -> Result<(usize, usize), Box<dyn Error>> {
    let path_str = path.as_ref().to_string_lossy();
    println!("Processing CSV file: {}", path_str);

    let mut success_count = 0;
    let mut error_count = 0;

    if path_str.contains("dream-dictionary") || path_str.contains("Dream Dictionary") {
        let records: Vec<DreamDictionaryRow> = read_csv_file(&path)?;
        println!("Read {} symbols from dream dictionary CSV", records.len());

        for record in records {
            if let (Some(symbol_name), Some(interpretation)) =
                (record.symbol, record.interpretation)
            {
                if symbol_name.trim().is_empty() || interpretation.trim().is_empty() {
                    continue;
                }

                let symbol =
                    row_to_symbol(&symbol_name, &interpretation, record.description.as_deref());

                match SymbolQueries::create(pool, &symbol).await {
                    Ok(_) => {
                        success_count += 1;
                    }
                    Err(DbError::Conflict(_)) => {
                        error_count += 1;
                    }
                    Err(err) => {
                        eprintln!("Error creating symbol {}: {}", symbol.name, err);
                        error_count += 1;
                    }
                }
            }
        }
    } else if path_str.contains("dictionary-of-dreams") || path_str.contains("Dictionary of Dreams")
    {
        let records: Vec<DictionaryOfDreamsRow> = read_csv_file(&path)?;
        println!(
            "Read {} symbols from Dictionary of Dreams CSV",
            records.len()
        );

        for record in records {
            if let (Some(symbol_name), Some(meaning)) = (record.symbol, record.meaning) {
                if symbol_name.trim().is_empty() || meaning.trim().is_empty() {
                    continue;
                }

                let symbol = row_to_symbol(&symbol_name, &meaning, record.description.as_deref());

                match SymbolQueries::create(pool, &symbol).await {
                    Ok(_) => {
                        success_count += 1;
                    }
                    Err(DbError::Conflict(_)) => {
                        error_count += 1;
                    }
                    Err(err) => {
                        eprintln!("Error creating symbol {}: {}", symbol.name, err);
                        error_count += 1;
                    }
                }
            }
        }
    } else if path_str.contains("dreams_interpretations")
        || path_str.contains("Dreams Interpretations")
    {
        let records: Vec<DreamsInterpretationsRow> = read_csv_file(&path)?;
        println!(
            "Read {} symbols from Dreams Interpretations CSV",
            records.len()
        );

        for record in records {
            if let (Some(symbol_name), Some(interpretation)) =
                (record.symbol, record.interpretation)
            {
                if symbol_name.trim().is_empty() || interpretation.trim().is_empty() {
                    continue;
                }

                let symbol = row_to_symbol(&symbol_name, &interpretation, None);

                match SymbolQueries::create(pool, &symbol).await {
                    Ok(_) => {
                        success_count += 1;
                    }
                    Err(DbError::Conflict(_)) => {
                        error_count += 1;
                    }
                    Err(err) => {
                        eprintln!("Error creating symbol {}: {}", symbol.name, err);
                        error_count += 1;
                    }
                }
            }
        }
    } else if path_str.contains("sample_dream_symbols") || path_str.contains("Sample Dream Symbols")
    {
        let records: Vec<SampleDreamSymbolRow> = read_csv_file(&path)?;
        println!(
            "Read {} symbols from Sample Dream Symbols CSV",
            records.len()
        );

        for record in records {
            if let (Some(symbol_name), Some(interpretation)) =
                (record.symbol, record.interpretation)
            {
                if symbol_name.trim().is_empty() || interpretation.trim().is_empty() {
                    continue;
                }

                let symbol =
                    row_to_symbol(&symbol_name, &interpretation, record.description.as_deref());

                match SymbolQueries::create(pool, &symbol).await {
                    Ok(_) => {
                        success_count += 1;
                    }
                    Err(DbError::Conflict(_)) => {
                        error_count += 1;
                    }
                    Err(err) => {
                        eprintln!("Error creating symbol {}: {}", symbol.name, err);
                        error_count += 1;
                    }
                }
            }
        }
    } else {
        let records: Vec<CleanedDreamInterpretationsRow> = match read_csv_file(&path) {
            Ok(records) => records,
            Err(err) => {
                eprintln!("Error reading CSV file {}: {}", path_str, err);
                return Ok((success_count, error_count));
            }
        };

        println!("Read {} symbols from generic CSV format", records.len());

        for record in records {
            if let (Some(word), Some(interpretation)) = (record.word, record.interpretation) {
                if word.trim().is_empty() || interpretation.trim().is_empty() {
                    continue;
                }

                let symbol = row_to_symbol(&word, &interpretation, None);

                match SymbolQueries::create(pool, &symbol).await {
                    Ok(_) => {
                        success_count += 1;
                    }
                    Err(DbError::Conflict(_)) => {
                        error_count += 1;
                    }
                    Err(err) => {
                        eprintln!("Error creating symbol {}: {}", symbol.name, err);
                        error_count += 1;
                    }
                }
            }
        }
    }

    println!(
        "CSV import complete: {} succeeded, {} errors",
        success_count, error_count
    );

    Ok((success_count, error_count))
}

async fn download_file(url: &str, output_path: &Path) -> Result<(), Box<dyn Error>> {
    println!("Downloading from: {}", url);
    let response = reqwest::get(url).await?;

    if !response.status().is_success() {
        return Err(format!("Failed to download file: HTTP {}", response.status()).into());
    }

    let bytes = response.bytes().await?;

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(output_path)?;
    file.write_all(&bytes)?;

    println!("Download complete: {}", output_path.display());
    Ok(())
}

fn extract_zip(zip_path: &Path, extract_to: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    println!("Extracting: {}", zip_path.display());

    fs::create_dir_all(extract_to)?;

    let file = File::open(zip_path)?;
    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader)?;

    let mut extracted_files = Vec::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = extract_to.join(file.name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;

            if outpath
                .extension()
                .map_or(false, |ext| ext == "csv" || ext == "json")
            {
                extracted_files.push(outpath.clone());
            }
        }
    }

    println!("Extracted {} files", extracted_files.len());
    Ok(extracted_files)
}

async fn reset_database(pool: &PgPool) -> Result<usize, Box<dyn Error>> {
    println!("Resetting database - clearing all symbols...");

    let symbols = SymbolQueries::list(pool, None).await?;
    println!("Found {} symbols to delete", symbols.len());

    let mut deleted_count = 0;

    for symbol in symbols {
        match SymbolQueries::delete(pool, &symbol.id).await {
            Ok(_) => {
                deleted_count += 1;
                if deleted_count % 100 == 0 {
                    println!("Deleted {} symbols...", deleted_count);
                }
            }
            Err(err) => {
                eprintln!("Error deleting symbol {}: {}", symbol.id, err);
            }
        }
    }

    println!(
        "Database reset complete. Deleted {} symbols.",
        deleted_count
    );
    Ok(deleted_count)
}

async fn download_kaggle_data(output_dir: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    println!("Downloading dream dictionary datasets from Kaggle...");

    fs::create_dir_all(output_dir)?;

    let dataset_urls = vec![(
        "https://storage.googleapis.com/kaggle-data-sets/cd/26683/bundle/archive.zip?X-Goog-Algorithm=GOOG4-RSA-SHA256&X-Goog-Credential=gcp-kaggle-com%40kaggle-161607.iam.gserviceaccount.com%2F20240620%2Fauto%2Fstorage%2Fgoog4_request&X-Goog-Date=20240620T144441Z&X-Goog-Expires=259200&X-Goog-SignedHeaders=host&X-Goog-Signature=72ca7fbca9ad5e10b4eafa7387e54cc83f0c9e9adcac6f9f7639a1ba7bc86efda1273a7ded8b9ad2d6ae02f4b3c9de1d7a30dd34b25bcb2d7908d43b7c304e2c6c00eb52efed2d7aedc9e81fefd1c1a90bcd34711ea9e0a15fddf1b87d58e8ecc6b9b0ef2c0afb5e89e98f4bc05ebc14a5d9e143f52c04b5dfd58d5c8cdba7242b66b79ed1c4cae80a4ba2e8a5a55cbb1c066c4dae05eb1ef3ba8864e8d67b9eac8b98f5ed1f7c7ccfa4a6c4dc77a1b19aeab4d2a22cd9b3b54ca2db5b2d1d17ebfbb36e0d7c7df7654e5b3a9e887c3ebe5bd2e78ab47a2f0dcb6d8ff7c05b8b1eeecb7b5e08f0dcf4d59b75caef53b8f909b26fa79f0fbe86",
        "dream_dictionary.zip",
    )];

    let mut all_files = Vec::new();

    for (url, filename) in dataset_urls {
        let zip_path = output_dir.join(filename);

        download_file(url, &zip_path).await?;

        let extract_dir = output_dir.join(filename.replace(".zip", ""));
        let extracted_files = extract_zip(&zip_path, &extract_dir)?;

        all_files.extend(extracted_files);
    }

    println!("Downloaded and extracted {} files", all_files.len());
    Ok(all_files)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let database_url = args.database_url.unwrap_or_else(|| {
        std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@localhost:5432/symbol_ontology".to_string()
        })
    });

    println!("Connecting to database: {}", database_url);
    let pool = create_pool(&database_url).await?;

    println!("Initializing database schema");
    init_database(&pool).await?;

    if args.reset {
        let deleted = reset_database(&pool).await?;
        println!("Database reset: {} symbols deleted", deleted);
    }

    let mut files_to_process = args.files.clone();

    if args.download {
        println!("Downloading datasets");
        let downloaded_files = download_kaggle_data(&args.output_dir).await?;
        files_to_process.extend(downloaded_files);
    }

    if args.test_data {
        println!("Seeding test data");
        SymbolQueries::seed_test_data(&pool).await?;
    }

    let mut total_success = 0;
    let mut total_errors = 0;

    for file_path in files_to_process {
        if !file_path.exists() {
            eprintln!("File not found: {}", file_path.display());
            continue;
        }

        let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");

        match ext.to_lowercase().as_str() {
            "json" => {
                println!("Processing JSON file: {}", file_path.display());
                let (success, errors) = import_json_file(&pool, &file_path).await?;
                total_success += success;
                total_errors += errors;
            }
            "csv" => {
                println!("Processing CSV file: {}", file_path.display());
                let (success, errors) = import_csv_file(&pool, &file_path).await?;
                total_success += success;
                total_errors += errors;
            }
            _ => {
                println!("Skipping unsupported file: {}", file_path.display());
            }
        }
    }

    println!("\n=== Symbol Ontology Seeder Summary ===");
    println!("Total symbols imported: {}", total_success);
    println!("Total errors: {}", total_errors);

    let all_symbols = SymbolQueries::list(&pool, None).await?;
    println!("Total symbols in database: {}", all_symbols.len());

    let mut categories = HashMap::new();
    for symbol in all_symbols {
        *categories.entry(symbol.category.clone()).or_insert(0) += 1;
    }

    println!("\nSymbols by category:");
    for (category, count) in categories {
        println!("  {}: {}", category, count);
    }

    Ok(())
}
