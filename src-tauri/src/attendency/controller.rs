use tokio::sync::Mutex;

use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use sqlx::postgres::PgRow;

use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sala {
    sala_id: i32,
    sala_piso: i32,
    sala_nom: String
}

impl Sala {
    fn sala_from_pg_row(row: &PgRow) -> Sala {
        let temp:Sala = Sala {
            sala_id: row.get("id"),
            sala_nom: row.get("nombre"),
            sala_piso: row.get("piso")
        };
        temp
    }

    fn vec_from_pg_row(vec: Vec<PgRow>) -> Vec<Sala> {
        let temp:Vec<Sala> = vec
        .iter()
        .map(Sala::sala_from_pg_row)
        .collect();
        temp
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asistencia {
    id: i32,
    sala_id: i32,
    visitante_id: i32,
    entrada: i64,
    salida: Option<i64>,
}

impl Asistencia {
    fn asistencia_from_pg_row(row: &PgRow) -> Asistencia {
        let temp:Asistencia = Asistencia {
            id: row.get("id"),
            sala_id: row.get("sala_id"),
            visitante_id: row.get("visitante_id"),
            entrada: row.get("entrada"),
            salida: row.get("salida"),
        };
        temp
    }

    fn vec_from_pg_row(vec: Vec<PgRow>) -> Vec<Asistencia> {
        let temp:Vec<Asistencia> = vec
        .iter()
        .map(Asistencia::asistencia_from_pg_row)
        .collect();
        temp
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterData {
    sala_id: i32,
    visitante_id: i32,
    fecha: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterAnswer {
    pub message: String,
    pub visitante_id: i32,
    pub register_type: String,
}

impl RegisterAnswer {
    pub fn new(message:String, visitante_id: i32, register_type: String) -> RegisterAnswer {
        let temp:RegisterAnswer = RegisterAnswer { message, visitante_id, register_type };
        temp
    }
}

#[tauri::command]
pub async fn get_salas(state: State<Mutex<Option<PgPool>>, '_>) -> Result<Vec<Sala>, ()> {
    let guard = state.lock().await;
    let pool = guard.as_ref().unwrap();

    let rows = sqlx::query("SELECT * FROM sala;")
        .fetch_all(pool)
        .await
        .unwrap();

    let salas: Vec<Sala> = Sala::vec_from_pg_row(rows);

    Ok(salas)
}

#[tauri::command]
pub async fn add_registration(
    state: State<Mutex<Option<PgPool>>, '_>,
    data: RegisterData,
) -> Result<RegisterAnswer, ()> {
    let guard = state.lock().await;
    let pool = guard.as_ref().unwrap();

    let added_user = add_user_to_database(pool, data.visitante_id).await;

    if added_user {
        println!("Added the user to our database");
    }

    let verify = sqlx::query("SELECT * FROM asistencia WHERE visitante_id = $1 AND salida IS NULL")
        .bind(data.visitante_id)
        .fetch_optional(pool)
        .await
        .unwrap();

    if verify.is_none() {
        let row = sqlx::query("insert into asistencia (sala_id, visitante_id, entrada) values ($1, $2, $3)")
        .bind(data.sala_id)
        .bind(data.visitante_id)
        .bind(data.fecha)
        .execute(pool)
        .await
        .unwrap();

        let mut answer:RegisterAnswer = RegisterAnswer::new("".to_string(), data.visitante_id, "Entrada".to_string());

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

        let mut answer:RegisterAnswer = RegisterAnswer::new("".to_string(), data.visitante_id, "Salida".to_string());

        if row.rows_affected() > 0 {
            answer.message = "Success".to_string();
            Ok(answer)
        } else {
            answer.message = "Failure".to_string();
            Ok(answer)
        }
    }
}

pub async fn add_user_to_database(pool: &PgPool, mat: i32) -> bool {
    let user = sqlx::query("select * from visitante where id = $1")
        .bind(mat)
        .fetch_optional(pool)
        .await
        .unwrap();

    if user.is_none() {
        let add_user = sqlx::query("insert into visitante (id) values ($1)")
            .bind(mat)
            .execute(pool)
            .await
            .unwrap();

        add_user.rows_affected() > 0
    } else {
        false
    }
}

#[tauri::command]
pub async fn get_statistics_by_date(
    state: State<Mutex<Option<PgPool>>, '_>,
    date: i64,
) -> Result<Vec<Asistencia>, ()> {
    let guard = state.lock().await;
    let db_pool = guard.as_ref().unwrap();

    let data = sqlx::query("select * from asistencia where entrada > $1 order by entrada")
        .bind(date)
        .fetch_all(db_pool)
        .await
        .unwrap();

    let rows:Vec<Asistencia> = Asistencia::vec_from_pg_row(data);

    Ok(rows)
}
