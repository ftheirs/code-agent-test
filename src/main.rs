use actix_web::{App, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{}:{}", host, port);

    println!("Starting server at http://{}", address);

    HttpServer::new(|| {
        App::new()
            // Add routes here later
    })
    .bind(&address)?
    .run()
    .await
}
