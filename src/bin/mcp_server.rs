use clap::Parser;
use dream_ontology_mcp::config::Config;
use dream_ontology_mcp::db::pool::create_pool;
use dream_ontology_mcp::db::repository::PgRepositoryFactory;
use dream_ontology_mcp::domain::Symbol;
use dream_ontology_mcp::logging::setup_logging;
use dream_ontology_mcp::mcp::service::SymbolService;
use rmcp::transport::sse_server::{SseServer, SseServerConfig};
use std::collections::HashMap;
use std::path::Path;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, default_value = "3002")]
    port: u16,

    #[clap(long)]
    database_url: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    setup_logging().expect("Failed to set up logging");

    let _config = Config::from_env(); // Not currently using config

    let database_url = args
        .database_url
        .or_else(|| std::env::var("DATABASE_URL").ok())
        .unwrap_or_else(|| "postgres://postgres:postgres@localhost/symbol_ontology".to_string());

    info!("Connecting to database at {}", database_url);

    let db_pool = match create_pool(&database_url).await {
        Ok(pool) => {
            info!("Successfully connected to PostgreSQL database");
            pool
        }
        Err(e) => {
            error!("Failed to connect to PostgreSQL database: {:?}", e);
            return Err(anyhow::anyhow!("Failed to connect to database: {}", e));
        }
    };

    match dream_ontology_mcp::db::pool::init_database(&db_pool).await {
        Ok(_) => info!("Database schema initialized"),
        Err(e) => {
            error!("Failed to initialize database schema: {:?}", e);
            return Err(anyhow::anyhow!(
                "Failed to initialize database schema: {}",
                e
            ));
        }
    }

    let factory = PgRepositoryFactory::new(db_pool.clone());
    let symbol_repository = factory.create_symbol_repository();

    let symbols = match symbol_repository.list_symbols(None).await {
        Ok(symbols) => {
            info!(
                "Successfully queried {} symbols from database",
                symbols.len()
            );

            if symbols.is_empty() {
                info!("No symbols found in database. Attempting to seed from JSON.");
                let json_path = Path::new("data/myth-symbol-seed.json");

                if json_path.exists() {
                    info!("Found JSON file. Importing symbols...");
                    let mut water_symbol = Symbol::new(
                        "water".to_string(),
                        "Water".to_string(),
                        "element".to_string(),
                        "Symbol of life, emotion, and purification".to_string(),
                    );

                    water_symbol.add_interpretation(
                        "dream".to_string(),
                        "Water in dreams may represent emotional states or the unconscious mind"
                            .to_string(),
                    );
                    water_symbol.add_related_symbol("ocean".to_string());
                    water_symbol.add_related_symbol("river".to_string());

                    if let Err(e) = symbol_repository.create_symbol(water_symbol).await {
                        warn!("Failed to create water symbol: {:?}", e);
                    }

                    vec![]
                } else {
                    warn!("JSON file not found: {}", json_path.display());
                    Vec::new()
                }
            } else {
                symbols
            }
        }
        Err(e) => {
            error!("Failed to query symbols from database: {:?}", e);
            Vec::new()
        }
    };

    let mut category_counts = HashMap::new();
    for symbol in &symbols {
        *category_counts.entry(symbol.category.clone()).or_insert(0) += 1;
    }

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(args.port);

    let bind_address = format!("0.0.0.0:{}", port);

    info!("=== Symbol Ontology MCP Server ===");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));
    info!("Loaded {} total symbols", symbols.len());

    let categories: Vec<String> = category_counts
        .iter()
        .map(|(cat, count)| format!("{}: {}", cat, count))
        .collect();
    info!("Categories: {}", categories.join(", "));

    info!("Starting server on {}", bind_address);
    info!("SSE endpoint: http://localhost:{}/sse", port);
    info!("Message endpoint: http://localhost:{}/message", port);
    info!("==============================");

    let config = SseServerConfig {
        bind: bind_address.parse()?,
        sse_path: "/sse".to_string(),
        post_path: "/message".to_string(),
        ct: CancellationToken::new(),
    };

    let db_pool_clone = db_pool.clone();
    let server = SseServer::serve_with_config(config)
        .await?
        .with_service(move || {
            info!("New client connected, creating SymbolService instance");
            SymbolService::new_with_db(db_pool_clone.clone())
        });

    info!("Server ready to accept connections");

    tokio::signal::ctrl_c().await?;
    server.cancel();
    info!("Server shutting down");

    Ok(())
}
