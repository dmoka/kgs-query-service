use fluent_asserter::prelude::*;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_that!(response.status().is_success()).is_true();
    assert_that!(response.content_length()).is_some_with_value(0);
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port(); // Retrieve the port assigned to us by the OS
    let server = kgs_query_service::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server); //Launch the server as a background task

    format!("http://127.0.0.1:{}", port)
}
