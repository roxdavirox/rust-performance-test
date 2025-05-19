use sqlx::{PgPool, Error};
use crate::models::{NovaPessoa, Pessoa};
use uuid::Uuid;

pub async fn criar_pessoa(pool: &PgPool, nova: NovaPessoa) -> Result<Pessoa, Error> {
    let id = Uuid::new_v4();

    let _ = sqlx::query(
        "INSERT INTO pessoas (id, apelido, nome, nascimento, stack) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(id)
    .bind(&nova.apelido)
    .bind(&nova.nome)
    .bind(&nova.nascimento)
    .bind(&nova.stack)
    .execute(pool)
    .await?;

    Ok(Pessoa {
        id,
        apelido: nova.apelido,
        nome: nova.nome,
        nascimento: nova.nascimento,
        stack: nova.stack,
    })
}

pub async fn buscar_pessoa_por_id(pool: &PgPool, id: Uuid) -> Result<Option<Pessoa>, Error> {
    let row = sqlx::query_as::<_, Pessoa>(
        "SELECT id, apelido, nome, nascimento, stack FROM pessoas WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn buscar_pessoas_por_termo(pool: &PgPool, termo: &str) -> Result<Vec<Pessoa>, Error> {
    let termo_like = format!("%{}%", termo);

    let pessoas = sqlx::query_as::<_, Pessoa>(
        r#"
        SELECT id, apelido, nome, nascimento, stack
        FROM pessoas
        WHERE nome ILIKE $1 OR apelido ILIKE $1 OR stack::text ILIKE $1
        LIMIT 50
        "#
    )
    .bind(termo_like)
    .fetch_all(pool)
    .await?;

    Ok(pessoas)
}

pub async fn contar_pessoas(pool: &PgPool) -> Result<i64, Error> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM pessoas")
        .fetch_one(pool)
        .await?;

    use sqlx::Row;
    Ok(row.get::<i64, _>("count"))
}

