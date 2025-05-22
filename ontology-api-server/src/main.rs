// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{routing::get, Router};
use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use ontology_core::db::pool::{create_pool, init_database};
use ontology_core::db::repository::PgRepositoryFactory;

mod api;
mod routes;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    /// Database URL
    #[arg(
        long,
        env("DATABASE_URL"),
        default_value = "postgres://postgres:postgres@localhost:5432/symbol_ontology"
    )]
    database_url: String,

    /// Log level
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize logging
    let level = match args.log_level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Symbol Ontology API Server starting up...");
    info!("Connecting to database: {}", args.database_url);

    // Initialize database connection
    let pool = create_pool(&args.database_url).await?;
    init_database(&pool).await?;

    // Create router with API routes (traditional REST API)
    let api_router = api::routes::router(pool.clone());

    // Create main router with both API routes
    let app = Router::new()
        .route("/", get(|| async { "Symbol Ontology API Server" }))
        .route("/health", get(|| async { "OK" }))
        .nest(
            "/api/v1",
            routes::create_api_router(Arc::new(PgRepositoryFactory::new(pool.clone()))),
        )
        .nest("/api/v2", api_router);

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
