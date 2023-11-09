use sqlx::{Postgres, PgConnection};
use sqlx_migrator::{error::Error, migration::Migration};
use sqlx_migrator::operation::Operation;
// Its better to use sqlx imported from sqlx_migrator
use sqlx_migrator::sqlx;

use super::create_table_sala::CTSalaM;
use super::create_table_visitant::CTVisitanteM;

pub(crate) struct CTAsistenciaO;

#[async_trait::async_trait]
impl Operation<Postgres> for CTAsistenciaO {
    // Up function runs apply migration
    async fn up(&self, connection: &mut PgConnection) -> Result<(), Error> {
        println!("asistencia");
        sqlx::query(
            "CREATE TABLE asistencia (
                id serial4 NOT NULL,
                sala_id int4 NULL,
                visitante_id int4 NULL,
                entrada int8 NOT NULL,
                salida int8 NULL,
                CONSTRAINT asistencia_pkey PRIMARY KEY (id),
                CONSTRAINT fk_sala FOREIGN KEY (sala_id) REFERENCES sala(id),
                CONSTRAINT fk_visitante FOREIGN KEY (visitante_id) REFERENCES visitante(id)
            );",
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    // down migration runs down migration
    async fn down(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query("DROP TABLE asistencia;")
            .execute(connection)
            .await?;
        Ok(())
    }
}

pub(crate) struct CTAsistenciaM;

#[async_trait::async_trait]
impl Migration<Postgres> for CTAsistenciaM {
    fn app(&self) -> &str {
        "0003"
    }

    fn name(&self) -> &str {
        "0003"
    }

    fn parents(&self) -> Vec<Box<dyn Migration<Postgres>>> {
        vec![
            Box::new(CTSalaM),
            Box::new(CTVisitanteM)
            ]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<Postgres>>> {
        vec![Box::new(CTAsistenciaO)]
    }
}
