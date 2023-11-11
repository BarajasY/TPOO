use sqlx::{postgres::PgRow, PgPool, Row};
use tokio::sync::Mutex;

use serde::{Deserialize, Serialize};

use tauri::State;

use crate::attendency::controller::RegisterAnswer;

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
  fecha: i64
}

impl Evento {
    fn evento_from_pg_row(row: &PgRow) -> Evento {
        let temp: Evento = Evento {
            id: row.get("id"),
            sala_id: row.get("sala_id"),
            nombre: row.get("nombre"),
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

    let row = sqlx::query("select * from eventos where id = $1")
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
    event: NewEvent,
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

    let verify = sqlx::query("select * from eventos_invitados where id= $1 and evento_id = $2")
        .bind(id)
        .bind(evento_id)
        .fetch_optional(db_pool)
        .await
        .unwrap();

    if verify.is_none() {
        let query = sqlx::query("insert into eventos_invitados (id, evento_id) values ($1, $2);")
            .bind(id)
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

    let added_user = add_user_to_database(pool, data.evento_invitados_id, data.evento_id).await;

    if added_user {
        println!("Added the user to our database");
    }

    let verify = sqlx::query("SELECT * FROM asistencia_eventos WHERE eventos_invitados_id = $1 AND salida IS NULL")
        .bind(data.evento_invitados_id)
        .fetch_optional(pool)
        .await
        .unwrap();

    if verify.is_none() {
        let row = sqlx::query(
            "insert into asistencia_eventos (evento_id, eventos_invitados_id, entrada) values ($1, $2, $3)",
        )
        .bind(data.evento_id)
        .bind(data.evento_invitados_id)
        .bind(data.fecha)
        .execute(pool)
        .await
        .unwrap();

        let mut answer: RegisterAnswer =
            RegisterAnswer::new("".to_string(), data.evento_invitados_id, "Entrada".to_string());

        if row.rows_affected() > 0 {
            answer.message = "Success".to_string();
            Ok(answer)
        } else {
            answer.message = "Failure".to_string();
            Ok(answer)
        }
    } else {
        let verify_id: i32 = verify.unwrap().get("id");
        let row = sqlx::query("update asistencia set salida = $1 where id = $2")
            .bind(data.fecha)
            .bind(verify_id)
            .execute(pool)
            .await
            .unwrap();

        let mut answer: RegisterAnswer =
            RegisterAnswer::new("".to_string(), data.evento_invitados_id, "Salida".to_string());

        if row.rows_affected() > 0 {
            answer.message = "Success".to_string();
            Ok(answer)
        } else {
            answer.message = "Failure".to_string();
            Ok(answer)
        }
    }
}

pub async fn add_user_to_database(pool: &PgPool, mat: i32, evento_id: i32) -> bool {
    let user = sqlx::query("select * from eventos_invitados where id = $1")
        .bind(mat)
        .fetch_optional(pool)
        .await
        .unwrap();

    if user.is_none() {
        let add_user = sqlx::query("insert into eventos_invitados (id, evento_id) values ($1, $2)")
            .bind(mat)
            .bind(evento_id)
            .execute(pool)
            .await
            .unwrap();

        add_user.rows_affected() > 0
    } else {
        false
    }
}
