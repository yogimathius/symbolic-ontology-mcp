use std::env;
use std::net::SocketAddr;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_addr: SocketAddr,
    pub database_url: Option<String>,
    pub seed_test_data: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_addr: SocketAddr::from(([127, 0, 0, 1], 3000)),
            database_url: None,
            seed_test_data: false,
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        println!("Loading configuration from environment variables...");

        let server_addr = env::var("SERVER_ADDR")
            .ok()
            .and_then(|addr| SocketAddr::from_str(&addr).ok())
            .unwrap_or_else(|| SocketAddr::from(([127, 0, 0, 1], 3000)));

        let database_url = env::var("DATABASE_URL").ok();
        if database_url.is_none() {
            println!(
                "WARNING: DATABASE_URL environment variable not set. Using default PostgreSQL connection string."
            );
        }

        let seed_test_data_str = env::var("SEED_TEST_DATA").unwrap_or_default();
        let seed_test_data =
            seed_test_data_str.to_lowercase() == "true" || seed_test_data_str == "1";

        Self {
            server_addr,
            database_url,
            seed_test_data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.server_addr.to_string(), "127.0.0.1:3000");
        assert_eq!(config.database_url, None);
        assert!(!config.seed_test_data);
    }
}
