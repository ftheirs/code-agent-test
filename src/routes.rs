use actix_web::{get, Responder};

// Placeholder handler function
async fn hello_world_handler() -> impl Responder {
    "Hello World!"
}

pub fn configure_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(index);
    cfg.service(hello);
}

#[get("/")]
async fn index() -> impl Responder {
    hello_world_handler().await
}

#[get("/hello")]
async fn hello() -> impl Responder {
    hello_world_handler().await
}
