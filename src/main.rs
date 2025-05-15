use axum::{routing::post, Router};
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr};
use num_cpus;

mod db;
mod models;
mod handlers;

fn main() {
	tokio::runtime::Builder::new_multi_thread()
		.worker_threads(num_cpus::get())
		.enable_all()
		.build()
		.unwrap()
		.block_on(async_main());
}

async fn async_main() {
	let pool = PgPoolOptions::new()
		.max_connections(100)
		.min_connections(50)
		.connect("postgres://postgres:password@db:5432/rinha")
		.await
		.expect("❌ Falha ao conectar no banco de dados");

    let app = Router::new()
        .route("/pessoas", post(handlers::post_pessoa))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    println!("🚀 Rodando em http://{}", addr);

	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.expect("❌ Erro ao iniciar o servidor");
}
