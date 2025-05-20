use api_rust::config::{self, AppConfig};

fn main() {
    let config = AppConfig::load().expect("Failed to load configuration");
    config::init_logger(&config.log_level);

    log::info!("Hello, Atlas!");
    println!("Hello, Atlas!");
}