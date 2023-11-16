use sqlx::{Postgres, PgConnection};
use sqlx_migrator::{error::Error, migration::Migration};
use sqlx_migrator::operation::Operation;
// Its better to use sqlx imported from sqlx_migrator
use sqlx_migrator::sqlx;

pub(crate) struct CTVisitanteO;

#[async_trait::async_trait]
impl Operation<Postgres> for CTVisitanteO {
    // Up function runs apply migration
    async fn up(&self, connection: &mut PgConnection) -> Result<(), Error> {
        println!("visitant");
        sqlx::query(
            "CREATE TABLE visitante (
                visitante_id SERIAL PRIMARY KEY,
                visitante_matricula INT,
                visitante_nombre VARCHAR(255),
                visitante_carrera VARCHAR(30),
                visitante_genero VARCHAR(50),
                visitante_semestre INT
            );",
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    // down migration runs down migration
    async fn down(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query("DROP TABLE visitante;")
            .execute(connection)
            .await?;
        Ok(())
    }
}

pub(crate) struct CTVisitanteM;

#[async_trait::async_trait]
impl Migration<Postgres> for CTVisitanteM {
    fn app(&self) -> &str {
        "0002"
    }

    fn name(&self) -> &str {
        "0002"
    }

    fn parents(&self) -> Vec<Box<dyn Migration<Postgres>>> {
        vec![]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<Postgres>>> {
        vec![Box::new(CTVisitanteO)]
    }
}
