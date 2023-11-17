use sqlx::{postgres::PgRow, PgPool, Row};
use tokio::sync::Mutex;

use serde::{Deserialize, Serialize};

use tauri::State;

use crate::attendency::controller::RegisterAnswer;
use crate::attendency::controller::AsistenciaData;
use crate::attendency::controller::get_visitante;

#[derive(Debug, Deserialize, Serialize)]
pub struct Evento {
  id: i32,
  sala_id: i32,
  nombre: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewEvent {
  sala_id: i32,
  nombre: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventAttendance {
  evento_id: i32,
  evento_invitados_id: i32,
  fecha: i64,
}

impl Evento {
  fn evento_from_pg_row(row: &PgRow) -> Evento {
      let temp: Evento = Evento {
        id: row.get("evento_id"),
        sala_id: row.get("evento_sala_id"),
        nombre: row.get("evento_nombre"),
      };
      temp
  }

  fn vec_from_pg_row(vec: Vec<PgRow>) -> Vec<Evento> {
    let temp: Vec<Evento> = vec.iter().map(Evento::evento_from_pg_row).collect();
    temp
  }
}

#[tauri::command]
pub async fn get_events(state: State<Mutex<Option<PgPool>>, '_>) -> Result<Vec<Evento>, ()> {
  let guard = state.lock().await;
  let db_pool = guard.as_ref().unwrap();

  let data = sqlx::query("select * from eventos")
    .fetch_all(db_pool)
    .await
    .unwrap();

  let rows: Vec<Evento> = Evento::vec_from_pg_row(data);

  Ok(rows)
}

#[tauri::command]
pub async fn get_event_by_id(
    state: State<Mutex<Option<PgPool>>, '_>,
    id: i32,
) -> Result<Evento, ()> {
  let guard = state.lock().await;
  let db_pool = guard.as_ref().unwrap();

  let row = sqlx::query("select * from eventos where evento_id = $1")
    .bind(id)
    .fetch_one(db_pool)
    .await
    .unwrap();

  let data = Evento::evento_from_pg_row(&row);

  Ok(data)
}

#[tauri::command]
pub async fn delete_event(state: State<Mutex<Option<PgPool>>, '_>, id: i32) -> Result<bool, ()> {
  let guard = state.lock().await;
  let db_pool = guard.as_ref().unwrap();

  let result = sqlx::query("delete from eventos where evento_id = $1")
    .bind(id)
    .execute(db_pool)
    .await
    .unwrap();

  Ok(result.rows_affected() > 0)
}

#[tauri::command]
pub async fn add_event(
    state: State<Mutex<Option<PgPool>>, '_>,
    event: NewEvent,
) -> Result<Option<Evento>, ()> {
  let guard = state.lock().await;
  let db_pool = guard.as_ref().unwrap();

  let row = sqlx::query("insert into eventos (evento_sala_id, evento_nombre) values($1, $2) returning *")
    .bind(event.sala_id)
    .bind(event.nombre)
    .fetch_optional(db_pool)
    .await
    .unwrap();

  match row {
      Some(row) => {
        let result: Evento = Evento::evento_from_pg_row(&row);
        Ok(Some(result))
      }
      None => Ok(None),
  }
}

#[tauri::command]
pub async fn add_invitado(
  state: State<Mutex<Option<PgPool>>, '_>,
  id: i32,
  evento_id: i32,
) -> Result<bool, ()> {
  let guard = state.lock().await;
  let db_pool = guard.as_ref().unwrap();

  let visitante = get_visitante(db_pool, id).await;

  let verify = sqlx::query("select * from eventos_invitados where visitante_id= $1 and eventos_id = $2")
    .bind(visitante.visitante_id)
    .bind(evento_id)
    .fetch_optional(db_pool)
    .await
    .unwrap();

  if verify.is_none() {
      let query = sqlx::query("insert into eventos_invitados (visitante_id, eventos_id) values ($1, $2);")
        .bind(visitante.visitante_id)
        .bind(evento_id)
        .execute(db_pool)
        .await
        .unwrap();
      Ok(query.rows_affected() > 0)
  } else {
      Ok(false)
  }
}

#[tauri::command]
pub async fn add_event_registration(
  state: State<Mutex<Option<PgPool>>, '_>,
  data: EventAttendance,
) -> Result<RegisterAnswer, ()> {
  dbg!(&data);
  let guard = state.lock().await;
  let pool = guard.as_ref().unwrap();

  let visitante = get_visitante(pool, data.evento_invitados_id).await;
  check_invitado(pool, visitante.visitante_id, data.evento_id).await;

  let verify = sqlx::query(
    "SELECT * FROM asistencia_data WHERE visitante_id = $1 AND salida IS NULL",
  )
  .bind(visitante.visitante_id)
  .fetch_optional(pool)
  .await
  .unwrap();

  if verify.is_none() {
    let row = sqlx::query(
        "insert into asistencia_data (visitante_id, entrada) values ($1, $2) returning *",
    )
    .bind(visitante.visitante_id)
    .bind(data.fecha)
    .fetch_one(pool)
    .await
    .unwrap();

    let asistencia_data:AsistenciaData = AsistenciaData::asistencia_data_from_pg_row(&row);
    let asistencia = sqlx::query("insert into asistencia (asistencia_evento_id, asistencia_asistencia_data_id) values ($1, $2)")
      .bind(data.evento_id)
      .bind(asistencia_data.asistencia_data_id)
      .execute(pool)
      .await
      .unwrap();

    let mut answer: RegisterAnswer = RegisterAnswer::new(
      "".to_string(),
      data.evento_invitados_id,
      "Entrada".to_string(),
    );

    if asistencia.rows_affected() > 0 {
      answer.message = "Success".to_string();
      Ok(answer)
    } else {
      answer.message = "Failure".to_string();
      Ok(answer)
    }
  } else {
    let verify_id: i32 = verify.unwrap().get("visitante_id");
    let row = sqlx::query("update asistencia_data set salida = $1 where visitante_id = $2")
      .bind(data.fecha)
      .bind(verify_id)
      .execute(pool)
      .await
      .unwrap();

    let mut answer: RegisterAnswer = RegisterAnswer::new(
      "".to_string(),
      data.evento_invitados_id,
      "Salida".to_string(),
    );

    if row.rows_affected() > 0 {
      answer.message = "Success".to_string();
      Ok(answer)
    } else {
      answer.message = "Failure".to_string();
      Ok(answer)
    }
  }
}

pub async fn check_invitado(pool: &PgPool, invitado_id: i32, evento_id: i32) -> bool {
  let verify = sqlx::query("select * from eventos_invitados where visitante_id= $1 and eventos_id = $2")
    .bind(invitado_id)
    .bind(evento_id)
    .fetch_optional(pool)
    .await
    .unwrap();

  if verify.is_none() {
      let query = sqlx::query("insert into eventos_invitados (visitante_id, eventos_id) values ($1, $2);")
        .bind(invitado_id)
        .bind(evento_id)
        .execute(pool)
        .await
        .unwrap();

      query.rows_affected() > 0
  } else {
      false
  }
}
