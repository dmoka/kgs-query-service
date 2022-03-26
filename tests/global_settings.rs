use bigdecimal::BigDecimal;
use fluent_asserter::prelude::*;
use kgs_query_service::configuration::{get_configuration, DatabaseSettings};
use kgs_query_service::routes::GlobalSettings;
use serde_json::json;
use sqlx::{Connection, Executor, PgConnection, PgPool};

use std::net::TcpListener;
use std::str::FromStr;
use uuid::Uuid;

/*continue from here
1. z2p line 58
4. we need to refactor the tests, so using a main and helper functions for example for spawn
5. telemetry
6. Oauth*/

/*
postgres cheatshet
\l - list database
\c - connect to db
\dt - show tables
*/

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

#[tokio::test]
async fn get_global_settings_config() {
    // Arrange
    let app = spawn_app().await;

    let id = uuid::Uuid::new_v4();
    let sql_insert = format!(
        "INSERT INTO global_settings (id,fuel_price,diesel_price) values('{}', {}, {})",
        id, 479, 481
    );

    sqlx::query(&sql_insert)
        .execute(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/global_settings", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_that!(response.status().is_success()).is_true();
    let body = response.text().await.unwrap();

    let global_setting_from_response: GlobalSettings = serde_json::from_str(&body).unwrap();

    assert_eq!(global_setting_from_response.id, id);
    assert_eq!(
        global_setting_from_response.fuel_price,
        BigDecimal::from(479)
    );
    assert_eq!(
        global_setting_from_response.diesel_price,
        BigDecimal::from(481)
    );

    /*let john = serde_json::json!({
        "id": id,
        "fuel_price": 479,
        "diesel_price": 481,
    });*/
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port(); // Retrieve the port assigned to us by the OS
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = uuid::Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    let server = kgs_query_service::startup::run(listener, connection_pool.clone())
        .expect("Failed to bind address");

    let _ = tokio::spawn(server); //Launch the server as a background task

    TestApp {
        address,
        db_pool: connection_pool,
    }
}
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");
    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}
