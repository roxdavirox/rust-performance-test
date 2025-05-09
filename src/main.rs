use axum::{routing::get, Router};
use std::net::SocketAddr;

async fn hello_world() -> &'static str {
    "Hello, world from Rust in Docker!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
