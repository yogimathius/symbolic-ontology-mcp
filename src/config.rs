use std::env;
use std::net::SocketAddr;
use std::str::FromStr;

/// Configuration for the application, loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    /// Address to bind the HTTP server to (default: 127.0.0.1:3000)
    pub server_addr: SocketAddr,
    /// Log level (default: info)
    pub log_level: String,
    /// Whether to use in-memory repository (default: true)
    pub use_memory_repository: bool,
    /// Database connection URL for PostgreSQL (default: None)
    pub database_url: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_addr: SocketAddr::from(([127, 0, 0, 1], 3000)),
            log_level: "info".to_string(),
            use_memory_repository: false,
            database_url: None,
        }
    }
}

impl Config {
    /// Load configuration from environment
    pub fn from_env() -> Self {
        println!("Loading configuration from environment variables...");

        let server_addr = env::var("SERVER_ADDR")
            .ok()
            .and_then(|addr| SocketAddr::from_str(&addr).ok())
            .unwrap_or_else(|| SocketAddr::from(([127, 0, 0, 1], 3000)));
        println!("SERVER_ADDR parsed as: {}", server_addr);

        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        println!("LOG_LEVEL parsed as: {}", log_level);

        let use_memory_str = env::var("USE_MEMORY_REPOSITORY").unwrap_or_default();
        println!("USE_MEMORY_REPOSITORY raw value: {}", use_memory_str);
        let use_memory_repository =
            use_memory_str.to_lowercase() == "true" || use_memory_str == "1";
        println!("USE_MEMORY_REPOSITORY parsed as: {}", use_memory_repository);

        let database_url = env::var("DATABASE_URL").ok();
        println!("DATABASE_URL parsed as: {:?}", database_url);

        Self {
            server_addr,
            log_level,
            use_memory_repository,
            database_url,
        }
    }

    /// Get the tracing filter based on the log level
    pub fn get_tracing_filter(&self) -> String {
        format!(
            "dream_ontology_mcp={},tower_http={}",
            self.log_level, self.log_level
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.server_addr.to_string(), "127.0.0.1:3000");
        assert_eq!(config.log_level, "info");
        assert!(config.use_memory_repository);
    }

    #[test]
    fn test_get_tracing_filter() {
        let config = Config {
            log_level: "debug".to_string(),
            ..Config::default()
        };
        assert_eq!(
            config.get_tracing_filter(),
            "dream_ontology_mcp=debug,tower_http=debug"
        );
    }
}
