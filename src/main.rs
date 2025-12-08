use actix_web::{App, HttpServer};
mod routes;
mod db;
mod services;
mod repositories;
mod models;
mod handlers;
mod errors;

use db::postgres::init_pg_pool;
use services::AppServices;
use std::fs;

fn setup_logger() -> Result<(), fern::InitError> {
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
            fern::log_file("logs/app.log")
                .expect("Failed to create log file")
        )
        .apply()?;
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logger().expect("Failed to initialize logger");
    log::info!("Starting NutriGo application");
    
    let pool = init_pg_pool().await;
    log::debug!("Database pool initialized");
    
    log::info!("Starting HTTP server on 127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(AppServices::new(pool.clone())))
            .configure(routes::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}