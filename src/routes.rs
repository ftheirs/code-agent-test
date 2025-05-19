/ src/routes.rs

use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(index))
    );
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}
