use reqwest::Client;
use std::{net::TcpListener};

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async {
        zero2prod::run(listener)
            .expect("Failed to bind address")
            .await
    });

    format!("http://127.0.0.1:{port}")
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request!");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscribe_returns_ok() {
    let address = spawn_app().await;
    let client: Client = Client::new();

    let body = "name=le%20gui&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(format!("{address}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to subscribe!");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscribe_err_on_missing_data() {
    let address = spawn_app().await;
    let client: Client = Client::new();

    let test_cases = vec![
        ("name=le%20gui", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{address}/subscriptions"))
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
