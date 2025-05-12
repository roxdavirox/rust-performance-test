use axum::{routing::post, Router};
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, time::Duration};
use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod db;
mod models;
mod handlers;

fn main() {
	tracing_subscriber::registry()
		.with(
			EnvFilter::from_default_env()
				.add_directive("axum=info".parse().unwrap())
				.add_directive("tower_http=info".parse().unwrap()), // opcional: detalha middleware
		)
		.with(tracing_subscriber::fmt::layer())
		.init();

	tokio::runtime::Builder::new_multi_thread()
		.worker_threads(num_cpus::get())
		.enable_all()
		.build()
		.unwrap()
		.block_on(async_main());
}

async fn async_main() {
	let pool = PgPoolOptions::new()
		.max_connections(200)
		.min_connections(0)
		.acquire_timeout(Duration::from_secs(10))
		.connect("postgres://postgres:password@db:5432/rinha")
		.await
		.expect("❌ Falha ao conectar no banco de dados");

	let app = Router::new()
		.route("/pessoas", post(handlers::post_pessoa))
		.with_state(pool)
		.layer(
			TraceLayer::new_for_http()
				.make_span_with(DefaultMakeSpan::new().include_headers(true))
				.on_response(DefaultOnResponse::new().include_headers(true)),
		);

	let addr = SocketAddr::from(([0, 0, 0, 0], 80));
	tracing::info!("🚀 Servidor rodando em http://{}", addr);

	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.expect("❌ Erro ao iniciar o servidor");
}
