use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use config::Config;
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Server is running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_by("info"));

    let settings = Config::builder()
        .add_source(config::Environment::with_prefix("APP"))
        .set_default("server_port", 8080)
        .unwrap()
        .build()
        .unwrap();

    let server_port = settings.get::<u16>("server_port").unwrap_or(8080);
    let server_address = format!("127.0.0.1:{}", server_port);

    log::info!("Starting server at {}", server_address);

    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(&server_address)?
    .run()
    .await
}
