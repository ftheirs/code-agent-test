use actix_web::{HttpResponse, Responder};

pub async fn hello_world_handler() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_hello_world_handler() {
        let resp = hello_world_handler().await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello World!");
    }
}
