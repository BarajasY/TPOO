use sqlx::{Postgres, PgConnection};
use sqlx_migrator::{error::Error, migration::Migration};
use sqlx_migrator::operation::Operation;
// Its better to use sqlx imported from sqlx_migrator
use sqlx_migrator::sqlx;

use super::create_table_asistencia_data::CTAsistenciaDataM;
use super::create_table_eventos::CTEventosM;
use super::create_table_sala::CTSalaM;

pub(crate) struct CTAsistenciaO;

#[async_trait::async_trait]
impl Operation<Postgres> for CTAsistenciaO {
    // Up function runs apply migration
    async fn up(&self, connection: &mut PgConnection) -> Result<(), Error> {
        println!("asistencia");
        sqlx::query(
            "CREATE TABLE asistencia (
                asistencia_id SERIAL PRIMARY KEY,
                asistencia_sala_id INT,
                asistencia_evento_id INT,
                asistencia_asistencia_data_id INT,
                CONSTRAINT fk_asistencia_asistencia_id FOREIGN KEY (asistencia_asistencia_data_id) REFERENCES asistencia_data(asistencia_data_id),
                CONSTRAINT fk_asistencia_sala_id FOREIGN KEY (asistencia_sala_id) REFERENCES sala(sala_id),
                CONSTRAINT fk_asistencia_evento_id FOREIGN KEY (asistencia_evento_id) REFERENCES eventos(evento_id)
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
        "0005"
    }

    fn name(&self) -> &str {
        "0005"
    }

    fn parents(&self) -> Vec<Box<dyn Migration<Postgres>>> {
        vec![
            Box::new(CTAsistenciaDataM),
            Box::new(CTSalaM),
            Box::new(CTEventosM),
            ]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<Postgres>>> {
        vec![Box::new(CTAsistenciaO)]
    }
}
