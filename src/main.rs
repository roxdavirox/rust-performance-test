use axum::{Router, routing::post};
use std::net::SocketAddr;

mod db;
mod models;
mod handlers;

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@db:5432/rinha")
        .await
        .expect("Erro ao conectar no banco");

    let app = Router::new()
        .route("/pessoas", post(handlers::post_pessoa))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    println!("🚀 Rodando em http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
