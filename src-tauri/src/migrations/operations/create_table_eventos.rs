use sqlx::{Postgres, PgConnection};
use sqlx_migrator::{error::Error, migration::Migration};
use sqlx_migrator::operation::Operation;
// Its better to use sqlx imported from sqlx_migrator
use sqlx_migrator::sqlx;
use super::create_table_sala::CTSalaM;

pub(crate) struct CTEventosO;

#[async_trait::async_trait]
impl Operation<Postgres> for CTEventosO {
    // Up function runs apply migration
    async fn up(&self, connection: &mut PgConnection) -> Result<(), Error> {
        println!("eventos");
        sqlx::query(
            "CREATE TABLE eventos (
              id serial4 NOT NULL,
              sala_id int4 NULL,
              nombre varchar(255) NULL,
              CONSTRAINT eventos_pkey PRIMARY KEY (id),
              CONSTRAINT fk_sala FOREIGN KEY (sala_id) REFERENCES sala(id)
            );",
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    // down migration runs down migration
    async fn down(&self, connection: &mut PgConnection) -> Result<(), Error> {
        sqlx::query("DROP TABLE eventos;")
            .execute(connection)
            .await?;
        Ok(())
    }
}

pub(crate) struct CTEventosM;

#[async_trait::async_trait]
impl Migration<Postgres> for CTEventosM {
    fn app(&self) -> &str {
        "0004"
    }

    fn name(&self) -> &str {
        "0004"
    }

    fn parents(&self) -> Vec<Box<dyn Migration<Postgres>>> {
        vec![Box::new(CTSalaM)]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<Postgres>>> {
        vec![Box::new(CTEventosO)]
    }
}
