use sqlx::{PgConnection, Postgres};
use sqlx_migrator::{error::Error, migration::Migration};
use sqlx_migrator::operation::Operation;
// Its better to use sqlx imported from sqlx_migrator
use sqlx_migrator::sqlx;

pub(crate) struct CTSalaO;

#[async_trait::async_trait]
impl Operation<Postgres> for CTSalaO {
    // Up function runs apply migration
    async fn up(&self, connection: &mut PgConnection) -> Result<(), Error> {
        println!("sala");
        sqlx::query(
            "CREATE TABLE sala (
                sala_id serial PRIMARY KEY,
                sala_piso int4 NULL,
                sala_nombre varchar(255) NULL
            );",
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    // down migration runs down migration
    async fn down(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query("DROP TABLE sala;")
            .execute(connection)
            .await?;
        Ok(())
    }
}

pub(crate) struct CTSalaM;

#[async_trait::async_trait]
impl Migration<Postgres> for CTSalaM {
    fn app(&self) -> &str {
        "0001"
    }

    fn name(&self) -> &str {
        "0001"
    }

    fn parents(&self) -> Vec<Box<dyn Migration<Postgres>>> {
        vec![]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<Postgres>>> {
        vec![Box::new(CTSalaO)]
    }
}
