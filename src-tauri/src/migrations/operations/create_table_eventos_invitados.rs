use sqlx::{Postgres, PgConnection};
use sqlx_migrator::{error::Error, migration::Migration};
use sqlx_migrator::operation::Operation;
// Its better to use sqlx imported from sqlx_migrator
use sqlx_migrator::sqlx;

use super::create_table_eventos::CTEventosM;

pub(crate) struct CTEventosInvitadosO;

#[async_trait::async_trait]
impl Operation<Postgres> for CTEventosInvitadosO {
    // Up function runs apply migration
    async fn up(&self, connection: &mut PgConnection) -> Result<(), Error> {
        println!("eventos invitados");
        sqlx::query(
            "CREATE TABLE eventos_invitados (
              id VARCHAR(255) NOT NULL,
              evento_id INT,
              CONSTRAINT eventos_invitados_pkey PRIMARY KEY (id),
              CONSTRAINT fk_evento FOREIGN KEY (evento_id) REFERENCES eventos(id)
            );",
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    // down migration runs down migration
    async fn down(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query("DROP TABLE eventos_invitados;")
            .execute(connection)
            .await?;
        Ok(())
    }
}

pub(crate) struct CTEventosInvitadosM;

#[async_trait::async_trait]
impl Migration<Postgres> for CTEventosInvitadosM {
    fn app(&self) -> &str {
        "0005"
    }

    fn name(&self) -> &str {
        "0005"
    }

    fn parents(&self) -> Vec<Box<dyn Migration<Postgres>>> {
        vec![Box::new(CTEventosM)]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<Postgres>>> {
        vec![Box::new(CTEventosInvitadosO)]
    }
}
