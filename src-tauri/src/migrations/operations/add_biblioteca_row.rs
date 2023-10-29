use sqlx::{Postgres, PgConnection};
use sqlx_migrator::{error::Error, migration::Migration};
use sqlx_migrator::operation::Operation;
// Its better to use sqlx imported from sqlx_migrator
use sqlx_migrator::sqlx;
use super::create_table_biblioteca::CTBibliotecaM;

pub(crate) struct ABibliotecaRO;

#[async_trait::async_trait]
impl Operation<Postgres> for ABibliotecaRO {
    // Up function runs apply migration
    async fn up(&self, connection: &mut PgConnection) -> Result<(), Error> {
        println!("biblioteca_add");
        sqlx::query(
            "INSERT INTO biblioteca (biblio_id) VALUES (1);",
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    // down migration runs down migration
    async fn down(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query("DELETE FROM biblioteca where biblio_id = 1;")
            .execute(connection)
            .await?;
        Ok(())
    }
}

pub(crate) struct ABibliotecaRM;

#[async_trait::async_trait]
impl Migration<Postgres> for ABibliotecaRM {
    fn app(&self) -> &str {
        "0005"
    }

    fn name(&self) -> &str {
        "0005"
    }

    fn parents(&self) -> Vec<Box<dyn Migration<Postgres>>> {
        vec![Box::new(CTBibliotecaM)]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<Postgres>>> {
        vec![Box::new(ABibliotecaRO)]
    }
}
