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
