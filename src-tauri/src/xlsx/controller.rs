use sqlx::postgres::PgRow;
use tauri::State;
use sqlx::{Row, PgPool};
use tokio::sync::Mutex;
use rust_xlsxwriter::*;
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime, NaiveDateTime};
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
pub struct AsistenciaDataJoinVisitante {
  asistencia_data_id: i32,
  visitante_id: i32,
  entrada: i64,
  salida: Option<i64>,
  visitante_matricula: i32,
}

impl AsistenciaDataJoinVisitante {
  fn asistencia_data_join_visitante_from_pg_row(row: &PgRow) -> AsistenciaDataJoinVisitante {
    let temp: AsistenciaDataJoinVisitante = AsistenciaDataJoinVisitante {
      asistencia_data_id: row.get("asistencia_data_id"),
      visitante_id: row.get("visitante_id"),
      entrada: row.get("entrada"),
      salida: row.get("salida"),
      visitante_matricula: row.get("visitante_matricula")
    };
    temp
}

fn vec_from_pg_row(vec: Vec<PgRow>) -> Vec<AsistenciaDataJoinVisitante> {
  let temp: Vec<AsistenciaDataJoinVisitante> = vec.iter().map(AsistenciaDataJoinVisitante::asistencia_data_join_visitante_from_pg_row).collect();
  temp
}
}

#[tauri::command]
pub async fn make_xlsx(
  state: State<Mutex<Option<PgPool>>, '_>,
  since: i64,
  path: String,
  name: String
) -> Result<bool, ()> {
  let guard = state.lock().await;
  let db_pool = guard.as_ref().unwrap();

  let asistencia_data = sqlx::query("select * from asistencia_data join visitante on visitante.visitante_id = asistencia_data.visitante_id where entrada > $1")
    .bind(since)
    .fetch_all(db_pool)
    .await
    .unwrap();

  let all_asistencia:Vec<AsistenciaDataJoinVisitante> = AsistenciaDataJoinVisitante::vec_from_pg_row(asistencia_data);

  let mut workbook = Workbook::new();

  let worksheet = workbook.add_worksheet();

  let mut count = 1;
  worksheet.write(0, 0, String::from("Id")).unwrap();
  worksheet.write(0, 1, String::from("Matricula")).unwrap();
  worksheet.write(0, 2, String::from("Entrada")).unwrap();
  worksheet.write(0, 3, String::from("Salida")).unwrap();
  for a in all_asistencia {
    //USE tauri shell COMMAND TO OPEN THE excel FOlDER ON CLICK
    worksheet.write(count, 0, a.asistencia_data_id).unwrap();
    worksheet.write(count, 1, a.visitante_matricula).unwrap();
    let dt = convert(a.entrada);
    worksheet.write(count, 2, dt.to_string()).unwrap();
    match a.salida {
      Some(b) => {
        let dt =  convert(b);
        worksheet.write(count, 3, dt.to_string()).unwrap()
      },
      None => worksheet.write(count, 3, 0).unwrap()
    };
    count += 1;
  }
  worksheet.autofit();

  let file = workbook.save(format!("{}/excel/{}.xlsx", path, name));

  Ok(file.is_ok())
}


pub fn convert(timestamp: i64)  -> DateTime<Utc> {
  let naive = NaiveDateTime::from_timestamp_opt(timestamp / 1000, (timestamp % 1000) as u32 * 1_000_000).unwrap();
  DateTime::<Utc>::from_utc(naive, Utc)
}
