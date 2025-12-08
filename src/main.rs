use actix_web::{App, HttpServer};
mod routes;
mod db;
mod services;
mod repositories;
mod models;
mod handlers;
mod errors;
mod config;

use db::postgres::init_pg_pool;
use services::AppServices;
use config::Config;
use std::fs;

fn setup_logger(log_file: &str) -> Result<(), fern::InitError> {
    // Create logs directory if it doesn't exist
    let _ = fs::create_dir("logs");

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(
            fern::log_file(log_file)
                .expect("Failed to create log file")
        )
        .apply()?;
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = Config::from_file("config.yaml")
        .expect("Failed to load configuration");
    
    // Setup logging with configured file path
    setup_logger(&config.logging.file_path).expect("Failed to initialize logger");
    log::info!("Starting NutriGo application");
    log::info!("Configuration loaded: {}@{}:{}", 
        config.database.username,
        config.database.host,
        config.database.port
    );
    
    let pool = init_pg_pool(&config).await;
    log::debug!("Database pool initialized");
    
    let addr = format!("{}:{}", config.server.host, config.server.port);
    log::info!("Starting HTTP server on {}", addr);
    
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(AppServices::new(pool.clone())))
            .configure(routes::configure)
    })
    .bind(&addr)?
    .run()
    .await
}