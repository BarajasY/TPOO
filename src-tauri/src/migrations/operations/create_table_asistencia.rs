use sqlx::{Postgres, PgConnection};
use sqlx_migrator::{error::Error, migration::Migration};
use sqlx_migrator::operation::Operation;
// Its better to use sqlx imported from sqlx_migrator
use sqlx_migrator::sqlx;

pub(crate) struct CTAsistenciaO;

#[async_trait::async_trait]
impl Operation<Postgres> for CTAsistenciaO {
    // Up function runs apply migration
    async fn up(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query(
            "CREATE TABLE asistencia (
              asistencia_id SERIAL PRIMARY KEY,
              sala_id int4 NOT NULL,
              visitant_mat int4 NOT NULL,
              biblio_id int4 NOT NULL,
              entrada int8 NULL,
              salida int8 NULL,
              CONSTRAINT fk_biblio_id FOREIGN KEY (biblio_id) REFERENCES biblioteca(biblio_id),
              CONSTRAINT fk_sala_id FOREIGN KEY (sala_id) REFERENCES sala(sala_id),
              CONSTRAINT fk_visitante_mat FOREIGN KEY (visitant_mat) REFERENCES visitant(visitant_mat)
            )",
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    // down migration runs down migration
    async fn down(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query("DROP TABLE asistencia")
            .execute(connection)
            .await?;
        Ok(())
    }
}

pub(crate) struct CTAsistenciaM;

#[async_trait::async_trait]
impl Migration<Postgres> for CTAsistenciaM {
    fn app(&self) -> &str {
        "main"
    }

    fn name(&self) -> &str {
        "create_table_asistencia"
    }

    fn parents(&self) -> Vec<Box<dyn Migration<Postgres>>> {
        vec![]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<Postgres>>> {
        vec![Box::new(CTAsistenciaO)]
    }
}
