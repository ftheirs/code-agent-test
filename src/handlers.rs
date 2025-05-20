use actix_web::{HttpResponse, Responder};

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use super::hello;

    #[actix_web::test]
    async fn test_hello_handler() {
        let resp = hello().await;
        assert_eq!(resp.status().as_u16(), 200);

        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello World!");
    }
}