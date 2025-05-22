use clap::{ArgAction, Parser};
use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use ontology_core::domain::Symbol;

#[derive(Parser, Debug)]
#[clap(author, version, about = "Seed the Symbol Ontology database")]
struct Args {
    #[clap(short, long, action=ArgAction::SetTrue)]
    reset: bool,

    #[clap(long)]
    database_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DreamSymbolRow {
    #[serde(rename = "Alphabet")]
    _alphabet: Option<String>,

    #[serde(rename = "Word")]
    word: Option<String>,

    #[serde(rename = "Interpretation")]
    interpretation: Option<String>,
}

fn extract_category(interpretation: &str) -> String {
    let interpretation_lower = interpretation.to_lowercase();

    let category_mappings = [
        ("dream", "dream"),
        ("nightmare", "nightmare"),
        ("spirit", "spiritual"),
        ("spiritual", "spiritual"),
        ("unconscious", "psychology"),
        ("psyche", "psychology"),
        ("feminine", "feminine"),
        ("masculine", "masculine"),
        ("shadow", "psychology"),
        ("chakra", "spiritual"),
        ("energy", "energy"),
        ("emotion", "emotional"),
        ("feeling", "emotional"),
        ("animal", "animal"),
        ("nature", "nature"),
        ("water", "elemental"),
        ("fire", "elemental"),
        ("earth", "elemental"),
        ("air", "elemental"),
        ("family", "relationship"),
        ("relationship", "relationship"),
        ("work", "work"),
        ("money", "material"),
        ("wealth", "material"),
        ("food", "nourishment"),
        ("health", "health"),
        ("death", "transformation"),
        ("rebirth", "transformation"),
    ];

    for (keyword, category) in category_mappings.iter() {
        if interpretation_lower.contains(keyword) {
            return category.to_string();
        }
    }

    "dream".to_string()
}

fn extract_related_symbols(interpretation: &str, all_symbols: &HashSet<String>) -> Vec<String> {
    let interpretation_lower = interpretation.to_lowercase();
    let mut related = HashSet::new();

    for symbol in all_symbols {
        let symbol_lower = symbol.to_lowercase();
        if interpretation_lower.contains(&symbol_lower)
            && (interpretation_lower.contains(&format!(" {}", symbol_lower))
                || interpretation_lower.contains(&format!("{} ", symbol_lower))
                || interpretation_lower.contains(&format!(".{}", symbol_lower))
                || interpretation_lower.contains(&format!("({}", symbol_lower)))
        {
            related.insert(symbol.clone());
        }
    }

    related.into_iter().collect()
}

fn extract_properties(_word: &str, interpretation: &str) -> HashMap<String, String> {
    let mut properties = HashMap::new();
    let interpretation_lower = interpretation.to_lowercase();

    if interpretation_lower.contains("positive")
        || interpretation_lower.contains("good omen")
        || interpretation_lower.contains("luck")
        || interpretation_lower.contains("fortune")
    {
        properties.insert("emotional_tone".to_string(), "positive".to_string());
    } else if interpretation_lower.contains("negative")
        || interpretation_lower.contains("bad omen")
        || interpretation_lower.contains("warning")
        || interpretation_lower.contains("danger")
    {
        properties.insert("emotional_tone".to_string(), "negative".to_string());
    } else {
        properties.insert("emotional_tone".to_string(), "neutral".to_string());
    }

    if interpretation_lower.contains("common dream")
        || interpretation_lower.contains("common symbol")
    {
        properties.insert("frequency".to_string(), "common".to_string());
    }

    if interpretation_lower.contains("nightmare") {
        properties.insert("dream_type".to_string(), "nightmare".to_string());
    } else if interpretation_lower.contains("lucid") {
        properties.insert("dream_type".to_string(), "lucid".to_string());
    }

    properties
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

async fn execute_simple_sql(db_url: &str, sql: &str) -> Result<(), Box<dyn Error>> {
    println!("Executing SQL: {}", sql);

    let (client, connection) = tokio_postgres::connect(db_url, tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    match client.simple_query(sql).await {
        Ok(_) => println!("  SQL executed successfully"),
        Err(e) => println!("  SQL error: {}", e),
    }

    Ok(())
}

async fn drop_tables(db_url: &str) -> Result<(), Box<dyn Error>> {
    println!("Dropping existing tables...");

    let drop_statements = [
        "DROP INDEX IF EXISTS idx_symbols_text_search",
        "DROP INDEX IF EXISTS idx_symbols_category",
        "DROP TABLE IF EXISTS symbol_sets",
        "DROP TABLE IF EXISTS symbols",
    ];

    for statement in drop_statements {
        execute_simple_sql(db_url, statement).await?;
    }

    println!("All tables dropped successfully");
    Ok(())
}

async fn create_tables(db_url: &str) -> Result<(), Box<dyn Error>> {
    println!("Creating database schema...");

    let statements = [
        "CREATE TABLE IF NOT EXISTS symbols (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            category TEXT NOT NULL,
            description TEXT NOT NULL,
            interpretations JSONB DEFAULT '{}'::JSONB,
            related_symbols JSONB DEFAULT '[]'::JSONB,
            properties JSONB DEFAULT '{}'::JSONB
        )",
        "CREATE TABLE IF NOT EXISTS symbol_sets (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            category TEXT NOT NULL,
            description TEXT NOT NULL,
            symbols JSONB DEFAULT '{}'::JSONB
        )",
        "CREATE INDEX IF NOT EXISTS idx_symbols_category ON symbols (category)",
        "CREATE INDEX IF NOT EXISTS idx_symbols_text_search 
         ON symbols USING GIN ((to_tsvector('english', name || ' ' || description)))",
    ];

    for statement in statements {
        execute_simple_sql(db_url, statement).await?;
    }

    println!("Schema creation complete");
    Ok(())
}

async fn get_symbol_count(db_url: &str) -> Result<usize, Box<dyn Error>> {
    println!("Counting symbols in database...");

    let (client, connection) = tokio_postgres::connect(db_url, tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let result = client.simple_query("SELECT COUNT(*) FROM symbols").await?;

    let mut count = 0;
    for msg in result {
        if let tokio_postgres::SimpleQueryMessage::Row(row) = msg {
            if let Some(count_str) = row.get(0) {
                if let Ok(n) = count_str.parse::<usize>() {
                    count = n;
                }
            }
        }
    }

    Ok(count)
}

async fn get_categories(db_url: &str) -> Result<HashMap<String, usize>, Box<dyn Error>> {
    println!("Getting categories from database...");

    let (client, connection) = tokio_postgres::connect(db_url, tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let result = client
        .simple_query("SELECT category, COUNT(*) FROM symbols GROUP BY category")
        .await?;

    let mut categories = HashMap::new();
    for msg in result {
        if let tokio_postgres::SimpleQueryMessage::Row(row) = msg {
            if let (Some(category), Some(count_str)) = (row.get(0), row.get(1)) {
                if let Ok(count) = count_str.parse::<usize>() {
                    categories.insert(category.to_string(), count);
                }
            }
        }
    }

    Ok(categories)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let database_url = "postgres://postgres:postgres@localhost:5432/symbol_ontology";

    let encoded_database_url = if database_url.contains('@') {
        let parts: Vec<&str> = database_url.splitn(2, '@').collect();
        let credentials_and_protocol = parts[0];
        let host_and_db = parts[1];

        if credentials_and_protocol.contains(':') {
            let cred_parts: Vec<&str> = credentials_and_protocol.splitn(3, ':').collect();
            let protocol = cred_parts[0];
            let username = cred_parts[1];
            let password = cred_parts[2]
                .replace("?", "%3F")
                .replace("&", "%26")
                .replace("=", "%3D");

            format!("{}:{}:{}@{}", protocol, username, password, host_and_db)
        } else {
            database_url.to_string()
        }
    } else {
        database_url.to_string()
    };

    println!("Connecting to database: {}", encoded_database_url);

    if args.reset {
        drop_tables(&encoded_database_url).await?;
    }

    create_tables(&encoded_database_url).await?;

    let csv_path = PathBuf::from("ontology-core/src/bin/data/dream_interpretations.csv");

    if !csv_path.exists() {
        eprintln!("File not found: {}", csv_path.display());
        return Ok(());
    }

    println!("Processing CSV file: {}", csv_path.display());

    let records: Vec<DreamSymbolRow> = read_csv_file(&csv_path)?;
    println!("Read {} symbols from CSV", records.len());

    let all_symbols: HashSet<String> = records
        .iter()
        .filter_map(|record| record.word.as_ref().map(|w| w.trim().to_string()))
        .collect();

    println!("Found {} unique symbol names", all_symbols.len());

    let mut symbols_to_process = Vec::new();

    for record in records {
        if let (Some(word), Some(interpretation)) =
            (record.word.as_ref(), record.interpretation.as_ref())
        {
            let word = word.trim();
            let interpretation = interpretation.trim();

            if word.is_empty() || interpretation.is_empty() {
                continue;
            }

            let id = word.to_lowercase().replace(" ", "_");

            let category = extract_category(interpretation);

            let related_symbols = extract_related_symbols(interpretation, &all_symbols);

            let properties = extract_properties(word, interpretation);

            let mut interpretations = HashMap::new();
            interpretations.insert("default".to_string(), interpretation.to_string());

            let symbol = Symbol {
                id,
                name: word.to_string(),
                category,
                description: interpretation.to_string(),
                interpretations,
                related_symbols,
                properties,
            };

            symbols_to_process.push(symbol);
        }
    }

    println!("Processing {} symbols", symbols_to_process.len());

    let (client, connection) =
        tokio_postgres::connect(&encoded_database_url, tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let mut success_count = 0;
    let mut error_count = 0;

    let batch_size = 20;

    for (batch_index, chunk) in symbols_to_process.chunks(batch_size).enumerate() {
        println!(
            "Processing batch {} ({} symbols)",
            batch_index + 1,
            chunk.len()
        );

        for symbol in chunk {
            let related_symbols_json = serde_json::to_string(&symbol.related_symbols)?;
            let interpretations_json = serde_json::to_string(&symbol.interpretations)?;
            let properties_json = serde_json::to_string(&symbol.properties)?;

            let sql = format!(
                "INSERT INTO symbols (id, name, category, description, interpretations, related_symbols, properties) 
                 VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}') 
                 ON CONFLICT (id) DO UPDATE SET 
                 name = EXCLUDED.name, 
                 category = EXCLUDED.category, 
                 description = EXCLUDED.description, 
                 interpretations = EXCLUDED.interpretations, 
                 related_symbols = EXCLUDED.related_symbols, 
                 properties = EXCLUDED.properties",
                symbol.id.replace("'", "''"),
                symbol.name.replace("'", "''"),
                symbol.category.replace("'", "''"),
                symbol.description.replace("'", "''"),
                interpretations_json.replace("'", "''"),
                related_symbols_json.replace("'", "''"),
                properties_json.replace("'", "''")
            );

            match client.simple_query(&sql).await {
                Ok(_) => {
                    success_count += 1;
                    if success_count % 20 == 0 {
                        println!("Successfully processed {} symbols", success_count);
                    }
                }
                Err(err) => {
                    eprintln!("Error creating symbol {}: {}", symbol.name, err);
                    error_count += 1;
                }
            }
        }
    }

    println!("\n=== Symbol Ontology Seeder Summary ===");
    println!("Total symbols processed: {}", symbols_to_process.len());
    println!("Total symbols imported: {}", success_count);
    println!("Total errors: {}", error_count);

    let symbol_count = get_symbol_count(&encoded_database_url).await?;
    println!("Total symbols in database: {}", symbol_count);

    let categories = get_categories(&encoded_database_url).await?;
    println!("\nSymbols by category:");
    for (category, count) in categories {
        println!("  {}: {}", category, count);
    }

    Ok(())
}
