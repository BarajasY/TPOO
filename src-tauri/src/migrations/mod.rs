use sqlx::Postgres;
use sqlx_migrator::migration::Migration;

pub(crate) mod operations;

pub(crate) fn migrations() -> Vec<Box<dyn Migration<Postgres>>> {
    vec![
        Box::new(operations::create_table_sala::CTSalaM),
        Box::new(operations::create_table_visitant::CTVisitanteM),
        Box::new(operations::create_table_asistencia::CTAsistenciaM),
        Box::new(operations::create_table_eventos::CTEventosM),
        Box::new(operations::create_table_eventos_invitados::CTEventosInvitadosM),
        Box::new(operations::create_table_asistencia_eventos::CTAsistenciaEventosM),
        Box::new(operations::create_table_estadisticas_cortas::CTEstadisticasCortasM),
        Box::new(operations::add_sala_row::ASalaRM),
    ]
}
