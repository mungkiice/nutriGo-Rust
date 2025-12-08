use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use crate::config::Config;

pub async fn init_pg_pool(config: &Config) -> Pool<Postgres> {
    log::debug!("Initializing PostgreSQL connection pool");
    
    let connection_string = config.database.connection_string();
    log::debug!("Connecting to database: {}@{}:{}", 
        config.database.username, 
        config.database.host, 
        config.database.port
    );
    
    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&connection_string)
        .await
        .expect("Failed to create pool");
    
    log::info!("PostgreSQL connection pool created successfully with {} max connections", 
        config.database.max_connections);
    pool
}