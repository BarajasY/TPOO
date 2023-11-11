use sqlx::{Postgres, PgConnection};
use sqlx_migrator::{error::Error, migration::Migration};
use sqlx_migrator::operation::Operation;
// Its better to use sqlx imported from sqlx_migrator
use sqlx_migrator::sqlx;

use super::create_table_eventos_invitados::CTEventosInvitadosM;
use super::create_table_eventos::CTEventosM;

pub(crate) struct CTAsistenciaEventosO;

#[async_trait::async_trait]
impl Operation<Postgres> for CTAsistenciaEventosO {
    // Up function runs apply migration
    async fn up(&self, connection: &mut PgConnection) -> Result<(), Error> {
        println!("asistencia eventos");
        sqlx::query(
            "CREATE TABLE asistencia_eventos (
              id Integer NOT NULL,
              evento_id INT,
              eventos_invitados_id Integer,
              entrada int8 NOT NULL,
              salida int8 NULL,
              CONSTRAINT asistencia_eventos_pkey PRIMARY KEY (id),
              CONSTRAINT fk_evento_invitado FOREIGN KEY (eventos_invitados_id) REFERENCES eventos_invitados(id),
              CONSTRAINT fk_evento FOREIGN KEY (evento_id) REFERENCES eventos(id)
            );",
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    // down migration runs down migration
    async fn down(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query("DROP TABLE asistencia_eventos;")
            .execute(connection)
            .await?;
        Ok(())
    }
}

pub(crate) struct CTAsistenciaEventosM;

#[async_trait::async_trait]
impl Migration<Postgres> for CTAsistenciaEventosM {
    fn app(&self) -> &str {
        "0006"
    }

    fn name(&self) -> &str {
        "0006"
    }

    fn parents(&self) -> Vec<Box<dyn Migration<Postgres>>> {
        vec![
          Box::new(CTEventosM),
          Box::new(CTEventosInvitadosM),
        ]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<Postgres>>> {
        vec![Box::new(CTAsistenciaEventosO)]
    }
}
