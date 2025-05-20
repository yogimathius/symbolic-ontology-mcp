// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_addr: String,
    pub database_url: Option<String>,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Self {
        let port = env::var("PORT").unwrap_or_else(|_| "3002".to_string());
        let server_addr = format!("0.0.0.0:{}", port);

        let database_url = env::var("DATABASE_URL").ok();
        let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        Config {
            server_addr,
            database_url,
            log_level,
        }
    }
}
