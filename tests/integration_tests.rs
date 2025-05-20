[cfg(test)]
mod integration_tests {
    use actix_web::{test, App, HttpServer};
    use actix_rt; // Required for the test attribute
    use reqwest; // Required for making HTTP requests in future tests
    use reqwest::StatusCode; // Required for asserting status codes
    use std::net::TcpListener; // To bind to an available port
    use log::info; // For logging within tests
    use env_logger; // For initializing logger in tests
    use tokio; // Required for tokio::spawn
    use reqwest::header::{self, HeaderMap, HeaderValue}; // Required for setting headers

    // Import main application setup (routes)
    use crate::routes::configure_routes;

    // Helper function to spawn the server for testing
    async fn spawn_server() -> String {
        // Initialize logger for tests (optional, but helpful for debugging)
        let _ = env_logger::builder().is_test(true).try_init();

        // Use 127.0.0.1:0 to let the OS choose a free port
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let addr = listener.local_addr().expect("Failed to get local address").to_string();

        info!("Test server binding to {}", addr);

        let server = HttpServer::new(|| {
            App::new()
                .configure(configure_routes) // Use the routes from the main app
        })
        .listen(listener) // Use the listener with the random port
        .expect("Failed to bind server")
        .run(); // This returns a Server handle

        // Spawn the server in a background task
        tokio::spawn(server);

        // Return the address the server is listening on
        addr
    }

    #[actix_rt::test]
    async fn test_get_root() {
        // Spawn the server
        let server_address = spawn_server().await;

        // Make a GET request to the root path
        let url = format!("http://{}/", server_address);
        let response = reqwest::get(&url).await.expect("Failed to send request to /");

        // Assert the status code is 200 OK
        assert_eq!(response.status(), StatusCode::OK);

        // Assert the response body is 'Hello World!'
        let body = response.text().await.expect("Failed to get response body from /");
        assert_eq!(body, "Hello World!");

        info!("GET / test passed");
    }

    #[actix_rt::test]
    async fn test_get_hello() {
        // Spawn the server
        let server_address = spawn_server().await;

        // Make a GET request to the /hello path
        let url = format!("http://{}/hello", server_address);
        let response = reqwest::get(&url).await.expect("Failed to send request to /hello");

        // Assert the status code is 200 OK
        assert_eq!(response.status(), StatusCode::OK);

        // Assert the response body is 'Hello World!'
        let body = response.text().await.expect("Failed to get response body from /hello");
        assert_eq!(body, "Hello World!");

        info!("GET /hello test passed (default plain text)");
    }

    #[actix_rt::test]
    async fn test_get_hello_json() {
        // Spawn the server
        let server_address = spawn_server().await;

        // Make a GET request to the /hello path with Accept: application/json header
        let url = format!("http://{}/hello", server_address);
        let client = reqwest::Client::new();
        let response = client.get(&url)
            .header(header::ACCEPT, HeaderValue::from_static("application/json"))
            .send()
            .await
            .expect("Failed to send request to /hello with Accept: application/json");

        // Assert the status code is 200 OK
        assert_eq!(response.status(), StatusCode::OK);

        // Assert the response body is the correct JSON string
        let body = response.text().await.expect("Failed to get response body from /hello (json)");
        assert_eq!(body, "{\"message\":\"Hello World!\"}");

        // Optionally, assert the Content-Type header
        assert_eq!(response.headers().get(header::CONTENT_TYPE).and_then(|v| v.to_str().ok()), Some("application/json"));

        info!("GET /hello test passed (JSON response)");
    }

    #[actix_rt::test]
    async fn test_get_hello_plain_text_with_header() {
        // Spawn the server
        let server_address = spawn_server().await;

        // Make a GET request to the /hello path with Accept: text/plain header
        let url = format!("http://{}/hello", server_address);
        let client = reqwest::Client::new();
        let response = client.get(&url)
            .header(header::ACCEPT, HeaderValue::from_static("text/plain"))
            .send()
            .await
            .expect("Failed to send request to /hello with Accept: text/plain");

        // Assert the status code is 200 OK
        assert_eq!(response.status(), StatusCode::OK);

        // Assert the response body is the correct plain text string
        let body = response.text().await.expect("Failed to get response body from /hello (plain text)");
        assert_eq!(body, "Hello World!");

        // Optionally, assert the Content-Type header (should be absent or text/plain)
        let content_type = response.headers().get(header::CONTENT_TYPE).and_then(|v| v.to_str().ok());
        assert!(content_type.is_none() || content_type == Some("text/plain"));

        info!("GET /hello test passed (plain text with header)");
    }
}