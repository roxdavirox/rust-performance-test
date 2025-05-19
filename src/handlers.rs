use axum::{
    extract::{Path, Query, State, Json},
    http::{header, StatusCode},
    response::IntoResponse,
    Json as AxumJson,
};
use crate::db;
use crate::models::{NovaPessoa, Pessoa};
use sqlx::PgPool;
use serde_json::json; 
use std::collections::HashMap;
use uuid::Uuid;

pub async fn post_pessoa(
    State(pool): State<PgPool>,
    Json(payload): Json<NovaPessoa>,
) -> impl IntoResponse {
    match db::criar_pessoa(&pool, payload).await {
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

pub async fn get_pessoa_by_id(
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    match db::buscar_pessoa_por_id(&pool, id).await {
        Ok(Some(p)) => (StatusCode::OK, AxumJson(p)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "não encontrado").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "erro interno").into_response(),
    }
}

pub async fn search_pessoas(
    Query(params): Query<HashMap<String, String>>,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let termo = match params.get("t") {
        Some(t) => t,
        None => return (StatusCode::BAD_REQUEST, "parametro 't' obrigatório").into_response(),
    };

    match db::buscar_pessoas_por_termo(&pool, termo).await {
        Ok(pessoas) => (StatusCode::OK, AxumJson(pessoas)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "erro interno").into_response(),
    }
}

pub async fn contagem_pessoas(
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    match db::contar_pessoas(&pool).await {
        Ok(count) => (StatusCode::OK, count.to_string()).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "erro interno").into_response(),
    }
}

