use actix_web::get;
use crate::handlers::hello_world_handler;

pub fn configure_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(index);
    cfg.service(hello);
}

#[get("/")]
async fn index() -> impl actix_web::Responder {
    hello_world_handler().await
}

#[get("/hello")]
async fn hello() -> impl actix_web::Responder {
    hello_world_handler().await
}
