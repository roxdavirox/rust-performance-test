use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("🚀 Listening on {}", addr);

    axum::serve(addr, app).await.unwrap();
}
