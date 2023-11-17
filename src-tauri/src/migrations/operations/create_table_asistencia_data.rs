use sqlx::{Postgres, PgConnection};
use sqlx_migrator::{error::Error, migration::Migration};
use sqlx_migrator::operation::Operation;
// Its better to use sqlx imported from sqlx_migrator
use sqlx_migrator::sqlx;

use super::create_table_visitante::CTVisitanteM;

pub(crate) struct CTAsistenciaDataO;

#[async_trait::async_trait]
impl Operation<Postgres> for CTAsistenciaDataO {
    // Up function runs apply migration
    async fn up(&self, connection: &mut PgConnection) -> Result<(), Error> {
        println!("asistencia data");
        sqlx::query(
            "CREATE TABLE asistencia_data (
              asistencia_data_id SERIAL PRIMARY KEY,
              visitante_id INT,
              entrada int8 NOT NULL,
              salida int8,
              CONSTRAINT fk_visitante_id FOREIGN KEY (visitante_id) REFERENCES visitante(visitante_id)
            );",
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    // down migration runs down migration
    async fn down(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query("DROP TABLE asistencia_data;")
            .execute(connection)
            .await?;
        Ok(())
    }
}

pub(crate) struct CTAsistenciaDataM;

#[async_trait::async_trait]
impl Migration<Postgres> for CTAsistenciaDataM {
    fn app(&self) -> &str {
        "0004"
    }

    fn name(&self) -> &str {
        "0004"
    }

    fn parents(&self) -> Vec<Box<dyn Migration<Postgres>>> {
        vec![
          Box::new(CTVisitanteM),
        ]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<Postgres>>> {
        vec![Box::new(CTAsistenciaDataO)]
    }
}
