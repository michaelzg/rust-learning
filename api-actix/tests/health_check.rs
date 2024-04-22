use std::net::TcpListener;

fn spawn_app() -> String {
    let host = "localhost";
    let listener = TcpListener::bind(format!("{host}:0")).expect("bind failed");
    let port = listener.local_addr().unwrap().port();
    let server = api_actix::run(listener).expect("server bringup failed");
    let _ = tokio::spawn(server);
    format!("http://{host}:{port}")
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("failed request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_success() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=bob&email=bob_2%40gmail.com";
    let response = client
        .post(format!("{address}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed request");
    assert_eq!(200, response.status().as_u16())
}

#[tokio::test]
async fn subscribe_rejected() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let tests = vec![
        ("missing email", "name=bob"),
        ("missing name", "email=bob2"),
        ("missing both email and name", "yo"),
    ];

    for (desc, body) in tests {
        let response = client.post(format!("{address}/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("failed request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "{} - failed with body {}", desc, body
        )
    }
}
