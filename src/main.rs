use actix_web::{App, HttpServer, web};
use log::info;
use dotenv::dotenv;

mod config;
mod routes;
mod handlers;
mod errors;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let app_config = config::Config::init();

    info!("Starting server at {}", &app_config.server.host);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_config.clone()))
            .configure(routes::configure)
    })
    .bind(&app_config.server.host)?
    .run()
    .await
}