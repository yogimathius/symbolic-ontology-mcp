// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

/*!
 * # Utilities Module
 *
 * Common utility functions used throughout the application.
 */

use std::path::PathBuf;

/// Get the configuration file path
pub fn get_config_path() -> PathBuf {
    std::env::var("ONTOLOGY_CONFIG_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
            path.push("symbol-ontology");
            path.push("config.toml");
            path
        })
}

/// Function to convert a string to lowercase
pub fn to_lowercase(s: &str) -> String {
    s.to_lowercase()
}

// Add more utility functions as needed
