#[cfg(test)]
mod integration_tests {
    use actix_web::{test, App, HttpServer};
    use actix_rt; // Required for the test attribute
    use reqwest; // Required for making HTTP requests in future tests
    use reqwest::StatusCode; // Required for asserting status codes
    use std::net::TcpListener; // To bind to an available port
    use log::info; // For logging within tests
    use env_logger; // For initializing logger in tests
    use tokio; // Required for tokio::spawn
    use reqwest::header::ACCEPT; // To set the Accept header

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
    async fn test_get_root_plain_text() {
        // Spawn the server
        let server_address = spawn_server().await;

        // Make a GET request to the root path without an Accept header
        let url = format!("http://{}/", server_address);
        let response = reqwest::get(&url).await.expect("Failed to send request to /");

        // Assert the status code is 200 OK
        assert_eq!(response.status(), StatusCode::OK);

        // Assert the response body is 'Hello World!'
        let body = response.text().await.expect("Failed to get response body from /");
        assert_eq!(body, "Hello World!");

        info!("GET / (plain text default) test passed");
    }

    #[actix_rt::test]
    async fn test_get_hello_plain_text() {
        // Spawn the server
        let server_address = spawn_server().await;

        // Make a GET request to the /hello path with Accept: text/plain
        let url = format!("http://{}/hello", server_address);
        let client = reqwest::Client::new();
        let response = client.get(&url)
            .header(ACCEPT, "text/plain")
            .send()
            .await
            .expect("Failed to send request to /hello");

        // Assert the status code is 200 OK
        assert_eq!(response.status(), StatusCode::OK);

        // Assert the response body is 'Hello World!'
        let body = response.text().await.expect("Failed to get response body from /hello");
        assert_eq!(body, "Hello World!");

        info!("GET /hello (plain text) test passed");
    }

    #[actix_rt::test]
    async fn test_get_hello_json() {
        // Spawn the server
        let server_address = spawn_server().await;

        // Make a GET request to the /hello path with Accept: application/json
        let url = format!("http://{}/hello", server_address);
        let client = reqwest::Client::new();
        let response = client.get(&url)
            .header(ACCEPT, "application/json")
            .send()
            .await
            .expect("Failed to send request to /hello with JSON accept header");

        // Assert the status code is 200 OK
        assert_eq!(response.status(), StatusCode::OK);

        // Assert the response body is the expected JSON string
        let body = response.text().await.expect("Failed to get response body from /hello (JSON)");
        assert_eq!(body, "{\"message\":\"Hello World!\"}");

        // Optionally, you could also deserialize the JSON to a struct and check its contents
        // let json_body: serde_json::Value = response.json().await.expect("Failed to deserialize JSON body");
        // assert_eq!(json_body["message"], "Hello World!");

        info!("GET /hello (JSON) test passed");
    }
}