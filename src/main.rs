use tokio::net::TcpListener;
use tracing::{debug, info};

use crate::config::Config;
use crate::db::pool::{create_pool, init_database};
use crate::db::queries::SymbolQueries;
use crate::logging::{setup_logging, trace_layer};

mod api;

mod config;

mod domain;

mod db;

mod logging;

mod mcp;

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dot_env_result = dotenvy::dotenv();

    match dot_env_result {
        Ok(path) => println!("Loaded .env from {}", path.display()),
        Err(e) => println!("Could not load .env file: {}", e),
    }

    setup_logging().expect("Failed to set up logging");

    let config = Config::from_env();

    info!(
        "Starting Dream Ontology Symbolic MCP Server v{}",
        utils::version()
    );

    debug!("Loaded configuration: {:?}", config);

    let database_url = config
        .database_url
        .clone()
        .expect("Database URL is required");
    info!("Connecting to database at {}", database_url);

    let db_pool = create_pool(&database_url).await?;

    info!("Initializing database schema");
    init_database(&db_pool).await?;

    if config.seed_test_data {
        info!("Seeding test data");
        SymbolQueries::seed_test_data(&db_pool).await.map_err(|e| {
            Box::<dyn std::error::Error>::from(format!("Error seeding test data: {}", e))
        })?;
    }

    let app = api::routes::router(db_pool.clone()).layer(trace_layer());

    debug!("API Server initialized with database connection and logging middleware");

    info!("API Server listening on {}", config.server_addr);

    let listener = TcpListener::bind(&config.server_addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    info!("Server shutting down");

    Ok(())
}
