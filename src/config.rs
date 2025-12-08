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

#[cfg(test)]
mod tests {
    use super::*;

    mod database_config {
        use super::*;

        #[test]
        fn connection_string_format_is_correct() {
            let config = DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                username: "user".to_string(),
                password: "pass".to_string(),
                database: "testdb".to_string(),
                max_connections: 5,
            };

            let expected = "postgres://user:pass@localhost:5432/testdb";
            assert_eq!(config.connection_string(), expected);
        }

        #[test]
        fn connection_string_with_special_characters() {
            let config = DatabaseConfig {
                host: "db.example.com".to_string(),
                port: 5433,
                username: "admin_user".to_string(),
                password: "p@ssw0rd".to_string(),
                database: "production_db".to_string(),
                max_connections: 10,
            };

            assert!(config.connection_string().contains("admin_user"));
            assert!(config.connection_string().contains("db.example.com"));
            assert!(config.connection_string().contains("5433"));
        }
    }

    mod server_config {
        use super::*;

        #[test]
        fn can_create_server_config() {
            let config = ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
            };

            assert_eq!(config.host, "0.0.0.0");
            assert_eq!(config.port, 8080);
        }

        #[test]
        fn supports_various_hosts() {
            let hosts = vec!["127.0.0.1", "0.0.0.0", "localhost", "example.com"];
            for host in hosts {
                let config = ServerConfig {
                    host: host.to_string(),
                    port: 8080,
                };
                assert_eq!(config.host, host);
            }
        }
    }

    mod logging_config {
        use super::*;

        #[test]
        fn can_create_logging_config() {
            let config = LoggingConfig {
                level: "info".to_string(),
                file_path: "logs/app.log".to_string(),
            };

            assert_eq!(config.level, "info");
            assert_eq!(config.file_path, "logs/app.log");
        }

        #[test]
        fn supports_various_log_levels() {
            let levels = vec!["trace", "debug", "info", "warn", "error"];
            for level in levels {
                let config = LoggingConfig {
                    level: level.to_string(),
                    file_path: "logs/app.log".to_string(),
                };
                assert_eq!(config.level, level);
            }
        }
    }

    mod config_default {
        use super::*;

        #[test]
        fn default_server_config() {
            let config = Config::default();
            assert_eq!(config.server.host, "127.0.0.1");
            assert_eq!(config.server.port, 8080);
        }

        #[test]
        fn default_database_config() {
            let config = Config::default();
            assert_eq!(config.database.host, "localhost");
            assert_eq!(config.database.port, 5432);
            assert_eq!(config.database.username, "nutrigo_rust_user");
            assert_eq!(config.database.database, "nutrigo_rust");
            assert_eq!(config.database.max_connections, 5);
        }

        #[test]
        fn default_logging_config() {
            let config = Config::default();
            assert_eq!(config.logging.level, "debug");
            assert_eq!(config.logging.file_path, "logs/app.log");
        }
    }
}
