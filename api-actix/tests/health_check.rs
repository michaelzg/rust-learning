use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let port = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://localhost:{}/health_check", port))
        .send()
        .await
        .expect("failed request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> u16 {
    let listener = TcpListener::bind("localhost:0").expect("bind failed");
    let port = listener.local_addr().unwrap().port();
    let server = api_actix::run(listener).expect("server bringup failed");
    let _ = tokio::spawn(server);
    port
}
