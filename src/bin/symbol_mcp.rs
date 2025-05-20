// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

use clap::Parser;
use tracing::{error, info};

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, default_value = "http://localhost:3002")]
    server: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    info!("Symbol MCP Client");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));
    info!("Using server: {}", args.server);

    // In a real client, we would connect to the server here

    info!("MCP client completed");
    Ok(())
}
