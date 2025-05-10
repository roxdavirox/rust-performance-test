use axum::{routing::get, Router};
use std::net::SocketAddr;
use hyper::Server;

#[tokio::main]
async fn main() {
  let app = Router::new().route("/", get(|| async { "Hello, world from Axum + Hyper!" }));
  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
  println!("🚀 Listening on {}", addr);

  Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}
