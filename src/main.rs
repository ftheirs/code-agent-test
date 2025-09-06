use api_rust::config::AppConfig;
use log::info;

fn main() -> std::io::Result<()> {
    // Load configuration
    let config = AppConfig::from_env().expect("Failed to load configuration");

    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(&config.log_level))
        .init();

    info!("Configuration loaded: {:?}", config);
    println!("Hello, Atlas!"); // Keep for Task 1 compatibility

    Ok(())
}
