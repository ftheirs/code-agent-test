#[cfg(test)]
mod integration_tests {
    use actix_web::{test, App, HttpServer};
    use actix_rt; // Required for the test attribute
    use reqwest; // Required for making HTTP requests in future tests
    use std::net::TcpListener; // To bind to an available port
    use log::info; // For logging within tests
    use env_logger; // For initializing logger in tests

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
    async fn test_server_spawns() {
        // Spawn the server
        let server_address = spawn_server().await;

        // Assert that the server address is not empty, indicating it started
        assert!(!server_address.is_empty());

        info!("Test server successfully spawned at: {}", server_address);

        // In future tasks, use reqwest to make requests to server_address
        // and assert responses.
    }
}