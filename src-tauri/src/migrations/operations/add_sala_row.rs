use sqlx::{Postgres, PgConnection};
use sqlx_migrator::{error::Error, migration::Migration};
use sqlx_migrator::operation::Operation;
// Its better to use sqlx imported from sqlx_migrator
use sqlx_migrator::sqlx;
use super::create_table_sala::CTSalaM;
use super::add_biblioteca_row::ABibliotecaRM;

pub(crate) struct ASalaRO;

#[async_trait::async_trait]
impl Operation<Postgres> for ASalaRO {
    // Up function runs apply migration
    async fn up(&self, connection: &mut PgConnection) -> Result<(), Error> {
        println!("sala_add");
        sqlx::query(
            "INSERT INTO sala (sala_id, sala_piso, sala_nom, biblio_id) VALUES (1, 1, 'Biblioteca piso 1', 1);",
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    // down migration runs down migration
    async fn down(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query("DELETE FROM sala where sala_id = 1;")
            .execute(connection)
            .await?;
        Ok(())
    }
}

pub(crate) struct ASalaRM;

#[async_trait::async_trait]
impl Migration<Postgres> for ASalaRM {
    fn app(&self) -> &str {
        "0006"
    }

    fn name(&self) -> &str {
        "0006"
    }

    fn parents(&self) -> Vec<Box<dyn Migration<Postgres>>> {
        vec![Box::new(CTSalaM),
            Box::new(ABibliotecaRM)]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<Postgres>>> {
        vec![Box::new(ASalaRO)]
    }
}
