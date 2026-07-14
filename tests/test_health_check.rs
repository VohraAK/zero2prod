use reqwest::Client;
use secrecy::ExposeSecret;
use sqlx::{Connection, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{DatabaseSettings, get_configuration};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection =
        PgConnection::connect(config.connection_string_without_db().expose_secret())
            .await
            .expect("Failed to connect to Postgres!");

    let create_db_query = format!(r#"CREATE DATABASE "{}";"#, config.database_name);
    sqlx::query(sqlx::AssertSqlSafe(create_db_query))
        .execute(&mut connection)
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect(config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database.");

    connection_pool
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{port}");

    let mut config = get_configuration().expect("Failed to load config!");
    config.database.database_name = Uuid::new_v4().to_string();

    let db_pool = configure_database(&config.database).await;

    let server = zero2prod::run(listener, db_pool.clone()).expect("Failed to bind address!");
    tokio::spawn(async move { server.await });

    TestApp { address, db_pool }
}

#[tokio::test]
async fn health_check_works() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", test_app.address))
        .send()
        .await
        .expect("Failed to execute request!");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscribe_returns_ok() {
    let test_app = spawn_app().await;

    let client: Client = Client::new();

    let body = "name=le%20gui&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(format!("{}/subscriptions", test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to subscribe!");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscribe_err_on_missing_data() {
    let test_app = spawn_app().await;
    let client: Client = Client::new();

    let test_cases = vec![
        ("name=le%20gui", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", test_app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to subscribe!");

        assert_eq!(
            422,
            response.status().as_u16(),
            "The API did not fail with 422 Unprocessable Entity when the payload was {}.",
            error_message
        );
    }
}
