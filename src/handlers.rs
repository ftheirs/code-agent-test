use actix_web::{HttpRequest, HttpResponse, Responder};
use serde_json::json;

pub async fn hello_world_handler(req: HttpRequest) -> impl Responder {
    let accepts_json = req.headers().get("Accept")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.contains("application/json"))
        .unwrap_or(false);

    if accepts_json {
        HttpResponse::Ok()
            .content_type("application/json")
            .body(json!({ "message": "Hello World!" }).to_string())
    } else {
        HttpResponse::Ok().body("Hello World!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App, http::header::ContentType};

    #[actix_web::test]
    async fn test_hello_world_handler_plain_text() {
        let req = test::TestRequest::default().to_http_request();
        let resp = hello_world_handler(req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello World!");
        assert_eq!(resp.headers().get("Content-Type"), None);
    }

    #[actix_web::test]
    async fn test_hello_world_handler_json() {
        let req = test::TestRequest::default()
            .header("Accept", "application/json")
            .to_http_request();
        let resp = hello_world_handler(req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        let body = test::read_body(resp).await;
        assert_eq!(body, "{\"message\":\"Hello World!\"}");
        assert_eq!(resp.headers().get("Content-Type"), Some(&ContentType::json().into()));
    }

    #[actix_web::test]
    async fn test_hello_world_handler_json_with_other() {
         let req = test::TestRequest::default()
            .header("Accept", "text/plain,application/json,q=0.9,*/*;q=0.8")
            .to_http_request();
        let resp = hello_world_handler(req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        let body = test::read_body(resp).await;
        assert_eq!(body, "{\"message\":\"Hello World!\"}");
        assert_eq!(resp.headers().get("Content-Type"), Some(&ContentType::json().into()));
    }
}
