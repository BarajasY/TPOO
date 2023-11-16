use tokio::sync::Mutex;

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};

use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sala {
  sala_id: i32,
  sala_piso: i32,
  sala_nom: String,
}

impl Sala {
  fn sala_from_pg_row(row: &PgRow) -> Sala {
    let temp: Sala = Sala {
      sala_id: row.get("sala_id"),
      sala_nom: row.get("sala_nombre"),
      sala_piso: row.get("sala_piso"),
    };
    temp
  }

  fn vec_from_pg_row(vec: Vec<PgRow>) -> Vec<Sala> {
    let temp: Vec<Sala> = vec.iter().map(Sala::sala_from_pg_row).collect();
    temp
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asistencia {
  asistencia_id: i32,
  sala_id: Option<i32>,
  evento_id: Option<i32>,
  asistencia_data_id: Option<i32>,
}

impl Asistencia {
  fn asistencia_from_pg_row(row: &PgRow) -> Asistencia {
    let temp: Asistencia = Asistencia {
      asistencia_id: row.get("asistencia_data_id"),
      sala_id: row.get("asistencia_sala_id"),
      evento_id: row.get("asistencia_evento_id"),
      asistencia_data_id: row.get("asistencia_asistencia_data_id"),
    };
    temp
  }

  fn vec_from_pg_row(vec: Vec<PgRow>) -> Vec<Asistencia> {
    let temp: Vec<Asistencia> = vec.iter().map(Asistencia::asistencia_from_pg_row).collect();
    temp
  }
}

pub struct Visitante {
  pub visitante_id: i32,
  pub visitante_matricula: i32,
  pub visitante_nombre: Option<String>,
  pub visitante_carrera: Option<String>,
  pub visitante_genero: Option<String>,
  pub visitante_semestre: Option<i32>,
}

impl Visitante {
  pub fn visitante_from_pg_row(row: &PgRow) -> Visitante {
    let temp: Visitante = Visitante {
      visitante_id: row.get("visitante_id"),
      visitante_matricula: row.get("visitante_matricula"),
      visitante_nombre: None,
      visitante_carrera: None,
      visitante_genero: None,
      visitante_semestre: None
    };
    temp
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AsistenciaData {
  pub asistencia_data_id: i32,
  pub visitante_id: i32,
  pub entrada: i64,
  pub salida: Option<i64>,
}

impl AsistenciaData {
  pub fn asistencia_data_from_pg_row(row: &PgRow) -> AsistenciaData {
    let temp: AsistenciaData = AsistenciaData {
      asistencia_data_id: row.get("asistencia_data_id"),
      visitante_id: row.get("visitante_id"),
      entrada: row.get("entrada"),
      salida: row.get("salida"),
    };
    temp
  }

  pub fn vec_from_pg_row(vec: Vec<PgRow>) -> Vec<AsistenciaData> {
    let temp: Vec<AsistenciaData> = vec.iter().map(AsistenciaData::asistencia_data_from_pg_row).collect();
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
  pub fn new(message: String, visitante_id: i32, register_type: String) -> RegisterAnswer {
    let temp: RegisterAnswer = RegisterAnswer {
        message,
        visitante_id,
        register_type,
    };
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

    let visitante = get_visitante(pool, data.visitante_id).await;

    let verify =
        sqlx::query("SELECT * FROM asistencia_data WHERE visitante_id = $1 AND salida IS NULL")
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

        let mut answer: RegisterAnswer =
        RegisterAnswer::new("".to_string(), data.visitante_id, "Entrada".to_string());

        let asistencia_data = AsistenciaData::asistencia_data_from_pg_row(&row);

        let asistencia = sqlx::query("insert into asistencia (asistencia_sala_id, asistencia_asistencia_data_id) values ($1, $2)")
          .bind(data.sala_id)
          .bind(asistencia_data.asistencia_data_id)
          .execute(pool)
          .await
          .unwrap();

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

        let mut answer: RegisterAnswer =
            RegisterAnswer::new("".to_string(), data.visitante_id, "Salida".to_string());

        if row.rows_affected() > 0 {
            answer.message = "Success".to_string();
            Ok(answer)
        } else {
            answer.message = "Failure".to_string();
            Ok(answer)
        }
    }
}

//Uses users matricula in order to get the rest of data from our database.
//Also inserts the user in the database incase it isn't already present.
pub async fn get_visitante(pool: &PgPool, mat: i32) -> Visitante {
  let user = sqlx::query("select * from visitante where visitante_matricula = $1")
    .bind(mat)
    .fetch_optional(pool)
    .await
    .unwrap();

  match user {
    Some(t) => Visitante::visitante_from_pg_row(&t),
    None => {
      let add_user = sqlx::query("insert into visitante (visitante_matricula) values ($1) returning *")
        .bind(mat)
        .fetch_one(pool)
        .await
        .unwrap();

      println!("Added user to our database");

      Visitante::visitante_from_pg_row(&add_user)
    }
  }
}

#[tauri::command]
pub async fn get_statistics_by_date(
    state: State<Mutex<Option<PgPool>>, '_>,
    date: i64,
) -> Result<Vec<AsistenciaData>, ()> {
    let guard = state.lock().await;
    let db_pool = guard.as_ref().unwrap();

    let data = sqlx::query("select * from asistencia_data where entrada > $1 order by entrada")
        .bind(date)
        .fetch_all(db_pool)
        .await
        .unwrap();

    let rows: Vec<AsistenciaData> = AsistenciaData::vec_from_pg_row(data);

    Ok(rows)
}

