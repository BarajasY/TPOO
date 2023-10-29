use sqlx_migrator::{error::Error, migration::Migration};
use sqlx_migrator::operation::Operation;
// Its better to use sqlx imported from sqlx_migrator
use sqlx::{Postgres, PgConnection};
use sqlx_migrator::sqlx;

pub(crate) struct CTBibliotecaO;

#[async_trait::async_trait]
impl Operation<Postgres> for CTBibliotecaO {
    // Up function runs apply migration
    async fn up(&self, connection: &mut PgConnection) -> Result<(), Error> {
        println!("biblioteca");
        sqlx::query(
            "CREATE TABLE biblioteca (
          biblio_id SERIAL PRIMARY KEY
        );",
        )
        .execute(connection)
        .await
        .expect("Could not create table biblioteca");
        Ok(())
    }

    // down migration runs down migration
    async fn down(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query("DROP TABLE biblioteca;")
            .execute(connection)
            .await
            .expect("Could not drop table biblioteca");
        Ok(())
    }
}

pub(crate) struct CTBibliotecaM;

#[async_trait::async_trait]
impl Migration<Postgres> for CTBibliotecaM {
    fn app(&self) -> &str {
        "001"
    }

    fn name(&self) -> &str {
        "0001"
    }

    fn parents(&self) -> Vec<Box<dyn Migration<Postgres>>> {
        vec![]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<Postgres>>> {
        vec![Box::new(CTBibliotecaO)]
    }
}
