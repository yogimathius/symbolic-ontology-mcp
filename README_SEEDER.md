# Dream Ontology Database Seeder

This tool allows you to seed your Dream Ontology database with entries from two Kaggle dream dictionary datasets:

1. [Dream Dictionary](https://www.kaggle.com/datasets/yuvrajsanghai/dream-dictionary)
2. [Dictionary of Dreams](https://www.kaggle.com/datasets/manswad/dictionary-of-dreams)

## Prerequisites

- Rust toolchain
- PostgreSQL database
- Database connection string available as `DATABASE_URL` environment variable or using the default connection string

## Usage Instructions

### Option 1: Quick Start with Sample Data

A sample dataset with 15 common dream symbols is included in this repository. You can use it to quickly seed your database:

```bash
cargo run --bin manual_seed data/sample_dream_symbols.csv
```

### Option 2: Full Dataset from Kaggle

For a comprehensive dream dictionary with hundreds of symbols:

#### Step 1: Download the CSV files

Manually download the CSV files from Kaggle:

1. Go to [Dream Dictionary](https://www.kaggle.com/datasets/yuvrajsanghai/dream-dictionary) and click "Download"
2. Go to [Dictionary of Dreams](https://www.kaggle.com/datasets/manswad/dictionary-of-dreams) and click "Download"
3. Extract the zip files to get the CSV files

#### Step 2: Run the seeder with the CSV files

```bash
cargo run --bin manual_seed path/to/dream_dictionary.csv path/to/dictionary_of_dreams.csv
```

For example:

```bash
cargo run --bin manual_seed ~/Downloads/Dream_Dictionary.csv ~/Downloads/DreamDic.csv
```

## Database Connection

By default, the tool connects to a PostgreSQL database at `postgres://postgres:postgres@localhost/symbol_ontology`. You can customize this by setting the `DATABASE_URL` environment variable:

```bash
export DATABASE_URL="postgres://username:password@host/database"
cargo run --bin manual_seed path/to/file1.csv path/to/file2.csv
```

## Data Transformation

The seeder will:

1. Transform CSV entries into symbols following the schema:

   - `symbol` → `name` and `id` (lowercase with spaces converted to underscores)
   - `category` → `"dream"`
   - `description` → fallback to `"No description"` if not available
   - `interpretations` → wrap original interpretation in a JSON object like `{ "default": "..." }`
   - `related_symbols` → left as `[]` for now
   - `properties` → empty object `{}` for now

2. Deduplicate entries based on the lowercase symbol name

3. Skip malformed rows with appropriate error logging

## Features

- **Automatic Format Detection**: The seeder can detect the CSV format even if the filename pattern isn't recognized
- **Error Tolerance**: Malformed rows are skipped rather than causing the entire process to fail
- **Deduplication**: Symbols with the same name (case-insensitive) are only imported once
- **Detailed Logging**: Progress and error information is displayed during the import process

## Troubleshooting

### Common Issues

1. **Database Connection Failed**: Make sure your PostgreSQL server is running and the credentials are correct.

2. **CSV Format Not Recognized**: Ensure the CSV files have the expected headers. The tool expects:

   - For Dream Dictionary: "Symbol" and "Interpretation" columns
   - For Dictionary of Dreams: "Symbol" and "Meaning" columns

3. **File Not Found**: Double-check the file paths provided to the command.

## Additional Resources

To expand your ontology with non-dream specific symbols, consider these other datasets:

- [ConceptNet](https://conceptnet.io/) - Common-sense knowledge graph
- [WordNet](https://wordnet.princeton.edu/) - Lexical database of English
- [Mythological Symbols Dataset](https://www.semanticscholar.org/paper/A-Knowledge-Graph-of-Mythological-Concepts-for-T%C3%B6pel-Filho/2cc92f0ef66e41c9936c) - Mythological and archetypal symbols
