use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn init_pg_pool() -> Pool<Postgres> {
    log::debug!("Initializing PostgreSQL connection pool");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://nutrigo_rust_user:nutrigorust@localhost/nutrigo_rust")
        .await
        .expect("Failed to create pool");
    
    log::info!("PostgreSQL connection pool created successfully");
    pool
}