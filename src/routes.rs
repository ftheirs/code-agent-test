use actix_web::web;
use super::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/").route(web::get().to(handlers::hello))
    );
    cfg.service(
        web::resource("/hello").route(web::get().to(handlers::hello))
    );
}