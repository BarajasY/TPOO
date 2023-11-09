use sqlx::{Postgres, PgConnection};
use sqlx_migrator::{error::Error, migration::Migration};
use sqlx_migrator::operation::Operation;
// Its better to use sqlx imported from sqlx_migrator
use sqlx_migrator::sqlx;

use super::create_table_eventos::CTEventosM;

pub(crate) struct CTEstadisticasCortasO;

#[async_trait::async_trait]
impl Operation<Postgres> for CTEstadisticasCortasO {
    // Up function runs apply migration
    async fn up(&self, connection: &mut PgConnection) -> Result<(), Error> {
        println!("estadisticas cortas");
        sqlx::query(
            "CREATE TABLE estadisticas_cortas (
              id VARCHAR(255) NOT NULL,
              entrada_frec VARCHAR(255),
              salida_frec VARCHAR(255),
              total_entradas INT,
              total_salidas INT,
              fecha Date default Now(),
              total_visitantes_reg INT,
              total_visitantes_hoy INT,
              CONSTRAINT estadisticas_cortas_pkey PRIMARY KEY (id)
            );",
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    // down migration runs down migration
    async fn down(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query("DROP TABLE estadisticas_cortas;")
            .execute(connection)
            .await?;
        Ok(())
    }
}

pub(crate) struct CTEstadisticasCortasM;

#[async_trait::async_trait]
impl Migration<Postgres> for CTEstadisticasCortasM {
    fn app(&self) -> &str {
        "0008"
    }

    fn name(&self) -> &str {
        "0008"
    }

    fn parents(&self) -> Vec<Box<dyn Migration<Postgres>>> {
        vec![Box::new(CTEventosM)]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<Postgres>>> {
        vec![Box::new(CTEstadisticasCortasO)]
    }
}
