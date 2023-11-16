use sqlx::Postgres;
use sqlx_migrator::migration::Migration;

pub(crate) mod operations;

pub(crate) fn migrations() -> Vec<Box<dyn Migration<Postgres>>> {
    vec![
        Box::new(operations::create_table_sala::CTSalaM),
        Box::new(operations::create_table_visitante::CTVisitanteM),
        Box::new(operations::create_table_asistencia::CTAsistenciaM),
        Box::new(operations::create_table_eventos::CTEventosM),
        Box::new(operations::create_table_asistencia_data::CTAsistenciaDataM),
        Box::new(operations::add_sala_row::ASalaRM),
        Box::new(operations::create_table_eventos_invitados::CTEventosInvitadosM),
    ]
}
