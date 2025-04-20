use csv::ReaderBuilder;
use reqwest;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio;
use zip::ZipArchive;

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
}

// New struct for dreams_interpretations.csv
#[derive(Debug, Deserialize)]
struct DreamsInterpretationsRow {
    #[serde(rename = "Dream Symbol")]
    symbol: Option<String>,
    #[serde(rename = "Interpretation")]
    interpretation: Option<String>,
}

// New struct for cleaned_dream_interpretations.csv and dream_interpretations_words.csv
#[derive(Debug, Deserialize)]
struct CleanedDreamInterpretationsRow {
    #[serde(rename = "Word")]
    word: Option<String>,
    #[serde(rename = "Interpretation")]
    interpretation: Option<String>,
}

// After the CleanedDreamInterpretationsRow struct, add a new struct for sample_dream_symbols.csv
// New struct for sample_dream_symbols.csv
#[derive(Debug, Deserialize)]
struct SampleDreamSymbolRow {
    #[serde(rename = "Symbol")]
    symbol: Option<String>,
    #[serde(rename = "Interpretation")]
    interpretation: Option<String>,
    #[serde(rename = "Description")]
    description: Option<String>,
}

// Download a file from URL
async fn download_file(url: &str, output_path: &Path) -> Result<(), Box<dyn Error>> {
    println!("Downloading from: {}", url);
    let response = reqwest::get(url).await?;

    if !response.status().is_success() {
        return Err(format!("Failed to download file: HTTP {}", response.status()).into());
    }

    let bytes = response.bytes().await?;

    // Create necessary directories
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write the file
    let mut file = File::create(output_path)?;
    file.write_all(&bytes)?;

    println!("Download complete: {}", output_path.display());
    Ok(())
}

// Extract a zip file
fn extract_zip(zip_path: &Path, extract_to: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    println!("Extracting: {}", zip_path.display());

    // Create output directory if it doesn't exist
    fs::create_dir_all(extract_to)?;

    let file = File::open(zip_path)?;
    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader)?;

    let mut extracted_files = Vec::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = extract_to.join(file.name());

        if file.name().ends_with('/') {
            // Create directory
            fs::create_dir_all(&outpath)?;
        } else {
            // Create parent directory if needed
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;

            // Only track CSV files
            if outpath.extension().map_or(false, |ext| ext == "csv") {
                extracted_files.push(outpath.clone());
            }
        }
    }

    println!("Extracted {} files", extracted_files.len());
    Ok(extracted_files)
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

                    let symbol = row_to_symbol(&symbol_name, &interpretation, None);

                    match symbol_repo.create_symbol(symbol).await {
                        Ok(_) => {
                            processed_symbols.insert(symbol_key);
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

            for record in records {
                if let (Some(word), Some(interpretation)) = (record.word, record.interpretation) {
                    if word.trim().is_empty() || interpretation.trim().is_empty() {
                        continue;
                    }

                    let symbol_key = word.to_lowercase();
                    if processed_symbols.contains(&symbol_key) {
                        continue;
                    }

                    let symbol = row_to_symbol(&word, &interpretation, None);

                    match symbol_repo.create_symbol(symbol).await {
                        Ok(_) => {
                            processed_symbols.insert(symbol_key);
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

/// Add thematic relationships between symbols based on domain knowledge
async fn enrich_symbol_relationships(
    symbol_repo: Arc<dyn SymbolRepository>,
) -> Result<(), Box<dyn Error>> {
    println!("Starting relationship enrichment process...");

    // Get all symbols from the database
    let symbols = symbol_repo.list_symbols(None).await?;
    println!(
        "Loaded {} symbols for relationship processing",
        symbols.len()
    );

    // Build a map for easy access
    let mut symbol_map: HashMap<String, Symbol> =
        symbols.into_iter().map(|s| (s.id.clone(), s)).collect();

    // Define thematic categories with their members
    // These groups define symbols that are semantically related
    let categories = vec![
        // Nature elements
        (
            "celestial",
            vec![
                "sun", "moon", "star", "sky", "heaven", "comet", "planet", "universe", "galaxy",
            ],
        ),
        (
            "water",
            vec![
                "ocean",
                "river",
                "lake",
                "sea",
                "rain",
                "flood",
                "swimming",
                "beach",
                "island",
                "boat",
                "ship",
                "water",
                "wave",
                "tide",
                "waterfall",
            ],
        ),
        (
            "earth",
            vec![
                "mountain",
                "hill",
                "valley",
                "cave",
                "rock",
                "stone",
                "dirt",
                "soil",
                "land",
                "earthquake",
                "volcano",
            ],
        ),
        (
            "plant",
            vec![
                "tree", "flower", "forest", "garden", "grass", "leaf", "root", "seed", "fruit",
                "woods",
            ],
        ),
        (
            "weather",
            vec![
                "storm",
                "rain",
                "thunder",
                "lightning",
                "wind",
                "hurricane",
                "tornado",
                "snow",
                "hail",
                "fog",
                "cloud",
            ],
        ),
        (
            "fire",
            vec![
                "flame",
                "burning",
                "ash",
                "smoke",
                "candle",
                "torch",
                "fireplace",
                "wildfire",
                "explosion",
                "fire",
            ],
        ),
        // Animals
        (
            "mammals",
            vec![
                "dog", "cat", "horse", "lion", "tiger", "bear", "wolf", "fox", "rabbit", "deer",
                "elephant", "monkey",
            ],
        ),
        (
            "reptiles",
            vec![
                "snake",
                "lizard",
                "turtle",
                "crocodile",
                "alligator",
                "dinosaur",
                "dragon",
            ],
        ),
        (
            "birds",
            vec![
                "eagle", "hawk", "owl", "crow", "raven", "dove", "peacock", "chicken", "swan",
                "bird",
            ],
        ),
        (
            "insects",
            vec![
                "spider",
                "ant",
                "bee",
                "butterfly",
                "mosquito",
                "fly",
                "beetle",
                "scorpion",
            ],
        ),
        (
            "aquatic",
            vec![
                "fish",
                "shark",
                "whale",
                "dolphin",
                "octopus",
                "jellyfish",
                "crab",
                "lobster",
            ],
        ),
        // Buildings/Structures
        (
            "home",
            vec![
                "house",
                "apartment",
                "mansion",
                "cabin",
                "cottage",
                "room",
                "attic",
                "basement",
                "kitchen",
            ],
        ),
        (
            "public",
            vec![
                "school", "hospital", "church", "temple", "castle", "prison", "tower", "building",
                "office",
            ],
        ),
        (
            "infrastructure",
            vec![
                "bridge", "road", "highway", "tunnel", "airport", "station", "wall", "fence",
                "gate",
            ],
        ),
        // Transport
        (
            "vehicles",
            vec![
                "car",
                "bus",
                "train",
                "airplane",
                "helicopter",
                "bicycle",
                "motorcycle",
                "boat",
                "ship",
                "rocket",
            ],
        ),
        // Body parts
        (
            "body",
            vec![
                "head", "face", "eye", "ear", "nose", "mouth", "tongue", "tooth", "hair", "hand",
                "arm", "leg", "foot", "heart", "blood", "bone",
            ],
        ),
        // Concepts
        (
            "time",
            vec![
                "clock", "watch", "calendar", "year", "month", "day", "night", "hour", "minute",
                "second", "past", "future",
            ],
        ),
        (
            "direction",
            vec![
                "north", "south", "east", "west", "up", "down", "left", "right", "center",
            ],
        ),
        (
            "color",
            vec![
                "red", "blue", "green", "yellow", "white", "black", "purple", "orange", "pink",
                "brown", "gray",
            ],
        ),
        (
            "emotions",
            vec![
                "love",
                "hate",
                "fear",
                "anger",
                "joy",
                "sadness",
                "anxiety",
                "peace",
                "confusion",
            ],
        ),
        (
            "light",
            vec![
                "light",
                "dark",
                "shadow",
                "sun",
                "moon",
                "star",
                "lamp",
                "candle",
                "flashlight",
                "darkness",
                "brightness",
            ],
        ),
        // Objects
        (
            "tools",
            vec![
                "hammer", "knife", "sword", "gun", "arrow", "axe", "spear", "shield", "key",
                "lock", "rope", "chain",
            ],
        ),
        (
            "clothing",
            vec![
                "dress",
                "shirt",
                "pants",
                "hat",
                "shoe",
                "boot",
                "coat",
                "jacket",
                "glove",
                "sock",
                "underwear",
                "costume",
            ],
        ),
        (
            "furniture",
            vec![
                "table", "chair", "bed", "couch", "desk", "shelf", "mirror", "clock", "lamp", "rug",
            ],
        ),
        (
            "communication",
            vec![
                "book",
                "letter",
                "phone",
                "computer",
                "television",
                "radio",
                "message",
                "email",
                "internet",
            ],
        ),
        (
            "food",
            vec![
                "bread",
                "meat",
                "fruit",
                "vegetable",
                "water",
                "milk",
                "wine",
                "beer",
                "coffee",
                "tea",
                "dinner",
                "breakfast",
            ],
        ),
        // Relationships
        (
            "family",
            vec![
                "mother",
                "father",
                "parent",
                "child",
                "baby",
                "brother",
                "sister",
                "grandparent",
                "family",
            ],
        ),
        (
            "social",
            vec![
                "friend", "enemy", "teacher", "student", "boss", "stranger", "neighbor", "crowd",
                "group",
            ],
        ),
        // Mythological
        (
            "mythical",
            vec![
                "angel", "demon", "god", "goddess", "devil", "ghost", "spirit", "monster",
                "vampire", "werewolf", "fairy", "witch", "wizard", "dragon",
            ],
        ),
    ];

    // Count of relationships added
    let mut relationship_count = 0;
    let mut symbol_updates = 0;

    // Track symbols in each category
    let mut category_members: HashMap<String, Vec<String>> = HashMap::new();

    // First pass: collect symbols by category
    for (category, members) in categories {
        let mut matched_symbols = Vec::new();

        for &member in members.iter() {
            // Exact match on ID
            if symbol_map.contains_key(member) {
                matched_symbols.push(member.to_string());
                continue;
            }

            // Check name contains search term
            for (id, symbol) in &symbol_map {
                // Skip if already matched
                if matched_symbols.contains(id) {
                    continue;
                }

                // Check if symbol name contains the category term
                let name_lower = symbol.name.to_lowercase();
                if name_lower.contains(member) || name_lower == member {
                    matched_symbols.push(id.clone());
                }
            }
        }

        if !matched_symbols.is_empty() {
            category_members.insert(category.to_string(), matched_symbols);
        }
    }

    // Second pass: create relationships within categories
    for (category, members) in &category_members {
        println!(
            "Processing category '{}' with {} members",
            category,
            members.len()
        );

        // Link all symbols in the same category (up to a reasonable limit)
        for symbol_id in members {
            if let Some(symbol) = symbol_map.get_mut(symbol_id) {
                // Get other category members
                let related: Vec<String> = members
                    .iter()
                    .filter(|&id| id != symbol_id)
                    .cloned()
                    .collect();

                // Take up to 5 related symbols to avoid overwhelming
                let related_subset = if related.len() > 5 {
                    related[0..5].to_vec()
                } else {
                    related
                };

                // Only proceed if we have new relationships to add
                if related_subset.is_empty() {
                    continue;
                }

                // Add new relationships (avoiding duplicates)
                let mut added_any = false;
                for related_id in &related_subset {
                    if !symbol.related_symbols.contains(related_id) {
                        symbol.related_symbols.push(related_id.clone());
                        relationship_count += 1;
                        added_any = true;
                    }
                }

                // Only update if we added new relationships
                if added_any {
                    // Update in database
                    match symbol_repo.update_symbol(symbol.clone()).await {
                        Ok(_) => {
                            symbol_updates += 1;
                            if symbol_updates % 50 == 0 {
                                println!("Updated {} symbols with relationships", symbol_updates);
                            }
                        }
                        Err(e) => eprintln!("Failed to update symbol {}: {}", symbol_id, e),
                    }
                }
            }
        }
    }

    println!("Relationship enrichment complete!");
    println!(
        "Added {} relationships across {} symbols",
        relationship_count, symbol_updates
    );
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get database URL from environment variable or use default
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/symbol_ontology".to_string());

    // Create data directory
    let data_dir = PathBuf::from("data");
    fs::create_dir_all(&data_dir)?;

    // URLs for the datasets - these are direct download links
    let urls = [
        // Note: These are examples. You'll need to replace with actual download links
        (
            "https://www.kaggle.com/api/v1/datasets/download/yuvrajsanghai/dream-dictionary",
            "dream-dictionary.zip",
        ),
        (
            "https://www.kaggle.com/api/v1/datasets/download/manswad/dictionary-of-dreams",
            "dictionary-of-dreams.zip",
        ),
    ];

    let mut csv_files = Vec::new();

    // Download and extract each dataset
    for (url, filename) in urls.iter() {
        let zip_path = data_dir.join(filename);

        // Skip downloading if file already exists
        if !zip_path.exists() {
            download_file(url, &zip_path).await?;
        } else {
            println!("File already exists: {}", zip_path.display());
        }

        // Extract the zip file
        let extracted = extract_zip(&zip_path, &data_dir)?;
        csv_files.extend(extracted);

        // Clean up the zip file
        fs::remove_file(zip_path)?;
    }

    // Add existing CSV files in the data directory
    for entry in fs::read_dir(&data_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "csv") {
            csv_files.push(path);
        }
    }

    // Process the CSV files and seed the database
    process_files_and_seed(&database_url, csv_files).await?;

    // Connect to the database for relationship enrichment
    println!("\nStarting relationship enrichment phase...");
    let factory = PostgresRepositoryFactory::new(&database_url).await?;
    let symbol_repo = factory.create_symbol_repository();

    // Enrich relationships
    enrich_symbol_relationships(symbol_repo).await?;

    println!("All processing complete!");
    Ok(())
}
