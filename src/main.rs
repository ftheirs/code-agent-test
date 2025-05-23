use actix_web::{App, HttpServer};
use env_logger;
use log::{info, LevelFilter};
use dotenv::dotenv;

mod config;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file if it exists. This should be done before loading config.
    dotenv().ok();

    let app_config = match config::AppConfig::load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            log::error!("Failed to load configuration: {}", e);
            // Exit the application if configuration loading fails
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to load configuration: {}", e),
            ));
        }
    };

    // Initialize logger based on configuration
    let log_level = app_config.log_level.parse::<LevelFilter>()
        .unwrap_or(LevelFilter::Info); // Fallback to Info if parsing fails

    env_logger::Builder::new()
        .filter_level(log_level)
        .init();

    // Add some test log messages at different levels
    log::trace!("This is a trace message (should only appear with trace log level)");
    log::debug!("This is a debug message (should appear with debug or trace log level)");
    log::info!("This is an info message (should appear with info, debug, trace log levels)");
    log::warn!("This is a warning message (should appear with warn, error, info, debug, trace log levels)");
    log::error!("This is an error message (should appear with error, warn, info, debug, trace log levels)");


    let bind_address = format!("{}:{}", app_config.host, app_config.port);
    info!("Starting Actix Web server on {}", bind_address);

    HttpServer::new(|| {
        App::new()
            .configure(routes::configure_routes)
    })
    .bind(&bind_address)?
    .run()
    .await
}