use sqlx::{Postgres, PgConnection};
use sqlx_migrator::{error::Error, migration::Migration};
use sqlx_migrator::operation::Operation;
// Its better to use sqlx imported from sqlx_migrator
use sqlx_migrator::sqlx;

pub(crate) struct ASalaRO;

#[async_trait::async_trait]
impl Operation<Postgres> for ASalaRO {
    // Up function runs apply migration
    async fn up(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO sala (sala_id, sala_piso, sala_nom, biblio_id) VALUES (1, 1, 'Biblioteca piso 1', 1)",
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    // down migration runs down migration
    async fn down(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query("DELETE FROM sala where sala_id = 1")
            .execute(connection)
            .await?;
        Ok(())
    }
}

pub(crate) struct ASalaRM;

#[async_trait::async_trait]
impl Migration<Postgres> for ASalaRM {
    fn app(&self) -> &str {
        "main"
    }

    fn name(&self) -> &str {
        "add_sala_row"
    }

    fn parents(&self) -> Vec<Box<dyn Migration<Postgres>>> {
        vec![]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<Postgres>>> {
        vec![Box::new(ASalaRO)]
    }
}
