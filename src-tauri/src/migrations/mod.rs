use sqlx::Postgres;
use sqlx_migrator::migration::Migration;

pub(crate) mod operations;

pub(crate) fn migrations() -> Vec<Box<dyn Migration<Postgres>>> {
    vec![
        Box::new(operations::create_table_biblioteca::CTBibliotecaM),
        Box::new(operations::create_table_sala::CTSalaM),
        Box::new(operations::create_table_visitant::CTVisitantM),
        Box::new(operations::create_table_asistencia::CTAsistenciaM),
        Box::new(operations::add_biblioteca_row::ABibliotecaRM),
        Box::new(operations::add_sala_row::ASalaRM),
    ]
}
