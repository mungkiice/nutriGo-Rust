use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file_path: String,
}

impl DatabaseConfig {
    /// Build the PostgreSQL connection string
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

impl Config {
    /// Load configuration from YAML file
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Load from environment variables first, then fall back to config file
        dotenv::dotenv().ok();

        if let Ok(config_str) = std::env::var("CONFIG_PATH") {
            Self::load_from_path(&config_str)
        } else if Path::new(path).exists() {
            Self::load_from_path(path)
        } else {
            log::warn!("Config file not found at {}, using default configuration", path);
            Ok(Self::default())
        }
    }

    fn load_from_path(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        log::info!("Configuration loaded from: {}", path);
        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        // Get values from environment variables or use defaults
        Self {
            server: ServerConfig {
                host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: std::env::var("SERVER_PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(8080),
            },
            database: DatabaseConfig {
                host: std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
                port: std::env::var("DB_PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(5432),
                username: std::env::var("DB_USER")
                    .unwrap_or_else(|_| "nutrigo_rust_user".to_string()),
                password: std::env::var("DB_PASSWORD")
                    .unwrap_or_else(|_| "nutrigorust".to_string()),
                database: std::env::var("DB_NAME").unwrap_or_else(|_| "nutrigo_rust".to_string()),
                max_connections: std::env::var("DB_MAX_CONNECTIONS")
                    .ok()
                    .and_then(|c| c.parse().ok())
                    .unwrap_or(5),
            },
            logging: LoggingConfig {
                level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| "debug".to_string()),
                file_path: std::env::var("LOG_FILE")
                    .unwrap_or_else(|_| "logs/app.log".to_string()),
            },
        }
    }
}
