use sqlx::{Postgres, PgConnection};
use sqlx_migrator::{error::Error, migration::Migration};
use sqlx_migrator::operation::Operation;
// Its better to use sqlx imported from sqlx_migrator
use sqlx_migrator::sqlx;

pub(crate) struct CTVisitantO;

#[async_trait::async_trait]
impl Operation<Postgres> for CTVisitantO {
    // Up function runs apply migration
    async fn up(&self, connection: &mut PgConnection) -> Result<(), Error> {
        println!("visitant");
        sqlx::query(
            "CREATE TABLE visitant (
                visitant_mat SERIAL PRIMARY KEY
            );",
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    // down migration runs down migration
    async fn down(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query("DROP TABLE visitant;")
            .execute(connection)
            .await?;
        Ok(())
    }
}

pub(crate) struct CTVisitantM;

#[async_trait::async_trait]
impl Migration<Postgres> for CTVisitantM {
    fn app(&self) -> &str {
        "0003"
    }

    fn name(&self) -> &str {
        "0003"
    }

    fn parents(&self) -> Vec<Box<dyn Migration<Postgres>>> {
        vec![]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<Postgres>>> {
        vec![Box::new(CTVisitantO)]
    }
}
