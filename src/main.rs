use axum::{routing::{post, get}, Router};
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr};
use num_cpus;

mod db;
mod models;
mod handlers;

fn main() {
	tokio::runtime::Builder::new_multi_thread()
		.worker_threads(2)
		.enable_all()
		.build()
		.unwrap()
		.block_on(async_main());
}

async fn async_main() {
	let pool = PgPoolOptions::new()
		.max_connections(40)
		.min_connections(10)
		.connect("postgres://postgres:password@db:5432/rinha")
		.await
		.expect("❌ Falha ao conectar no banco de dados");

    let app = Router::new()
        .route("/pessoas", post(handlers::post_pessoa))
        .route("/pessoas", get(handlers::search_pessoas))
        .route("/pessoas/:id", get(handlers::get_pessoa_by_id))
        .route("/contagem-pessoas", get(handlers::contagem_pessoas))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    println!("🚀 Rodando em http://{}", addr);

	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.expect("❌ Erro ao iniciar o servidor");
}
