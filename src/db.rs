use sqlx::{PgPool, Error};
use crate::models::{NovaPessoa, Pessoa};
use uuid::Uuid;

pub async fn criar_pessoa(pool: &PgPool, nova: NovaPessoa) -> Result<Pessoa, Error> {
  let id = Uuid::new_v4();

  // Desestrutura os campos (claro e simples)
  let NovaPessoa {
      apelido,
      nome,
      nascimento,
      stack,
  } = nova;

  sqlx::query(
      "INSERT INTO pessoas (id, apelido, nome, nascimento, stack) VALUES ($1, $2, $3, $4, $5)",
  )
  .bind(id)
  .bind(&apelido)
  .bind(&nome)
  .bind(&nascimento)
  .bind(&stack)
  .execute(pool)
  .await?;

  Ok(Pessoa {
      id,
      apelido,
      nome,
      nascimento,
      stack,
  })
}
