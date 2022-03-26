use fluent_asserter::prelude::*;
use kgs_query_service::configuration::get_configuration;
use sqlx::PgPool;
use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_that!(response.status().is_success()).is_true();
    assert_that!(response.content_length()).is_some_with_value(0);
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port(); // Retrieve the port assigned to us by the OS
    let address = format!("http://127.0.0.1:{}", port);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let server = kgs_query_service::startup::run(listener, connection_pool.clone())
        .expect("Failed to bind address");

    let _ = tokio::spawn(server); //Launch the server as a background task

    TestApp {
        address,
        db_pool: connection_pool,
    }
}
