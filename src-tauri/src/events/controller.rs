use sqlx::{Row, postgres::PgRow, PgPool};
use tokio::sync::Mutex;

use serde::{Deserialize, Serialize};

use tauri::State;

#[derive(Debug, Deserialize, Serialize)]
pub struct Evento {
  id: i32,
  sala_id: i32,
  nombre: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewEvent {
  sala_id: i32,
  nombre: String
}

impl Evento {
  fn evento_from_pg_row(row: &PgRow) -> Evento {
    let temp:Evento = Evento {
      id: row.get("id"),
      sala_id: row.get("sala_id"),
      nombre: row.get("nombre")
    };
    temp
}

fn vec_from_pg_row(vec: Vec<PgRow>) -> Vec<Evento> {
    let temp:Vec<Evento> = vec
    .iter()
    .map(Evento::evento_from_pg_row)
    .collect();
    temp
}
}

#[tauri::command]
pub async fn get_events(
    state: State<Mutex<Option<PgPool>>, '_>
) -> Result<Vec<Evento>, ()> {
    let guard = state.lock().await;
    let db_pool = guard.as_ref().unwrap();

    let data = sqlx::query("select * from eventos")
        .fetch_all(db_pool)
        .await
        .unwrap();

    let rows:Vec<Evento> = Evento::vec_from_pg_row(data);

    Ok(rows)
}

#[tauri::command]
pub async fn delete_event(
  state: State<Mutex<Option<PgPool>>, '_>,
  id: i32
) -> Result<bool, ()> {
  let guard = state.lock().await;
  let db_pool = guard.as_ref().unwrap();

  let result = sqlx::query("delete from eventos where id = $1")
    .bind(id)
    .execute(db_pool)
    .await
    .unwrap();

  Ok(result.rows_affected() > 0)
}

#[tauri::command]
pub async fn add_event(
  state: State<Mutex<Option<PgPool>>, '_>,
  event: NewEvent
) -> Result<Option<Evento>, ()> {
  let guard = state.lock().await;
  let db_pool = guard.as_ref().unwrap();

  let row = sqlx::query("insert into eventos (sala_id, nombre) values($1, $2) returning *")
    .bind(event.sala_id)
    .bind(event.nombre)
    .fetch_optional(db_pool)
    .await
    .unwrap();

  match row {
    Some(row) => {
      let result:Evento = Evento::evento_from_pg_row(&row);
      Ok(Some(result))
    },
    None => {
      Ok(None)
    }
  }
}
