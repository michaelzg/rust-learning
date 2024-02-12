use api_actix::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("localhost:8000").expect("failed to bind");
    run(listener).expect("run failed").await
}
