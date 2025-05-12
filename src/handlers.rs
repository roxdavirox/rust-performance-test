use axum::{
    extract::{State, Json},
    http::{header, StatusCode},
    response::IntoResponse,
    Json as AxumJson,
};
use crate::{db::criar_pessoa, models::NovaPessoa};
use sqlx::PgPool;
use serde_json::json;

#[tracing::instrument(name = "POST /pessoas", skip(pool, payload))]
pub async fn post_pessoa(
    State(pool): State<PgPool>,
    Json(payload): Json<NovaPessoa>,
) -> impl IntoResponse {
    match criar_pessoa(&pool, payload).await {
        Ok(p) => (
            StatusCode::CREATED,
            [(header::LOCATION, format!("/pessoas/{}", p.id))],
            AxumJson(json!({ "id": p.id.to_string() })),
        )
            .into_response(),

        Err(e) if e.to_string().contains("duplicate key") => (
            StatusCode::UNPROCESSABLE_ENTITY,
            "apelido já existe",
        )
            .into_response(),

        Err(_) => (
            StatusCode::BAD_REQUEST,
            "erro ao inserir",
        )
            .into_response(),
    }
}
