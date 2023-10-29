use serde::{Deserialize, Serialize};
use sqlx::{Row, PgPool};
use tauri::State;

use crate::Database;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sala {
    sala_id: i32,
    sala_piso: i32,
    sala_nom: String,
    biblio_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asistencia {
    sala_id: i32,
    visitante_mat: i32,
    biblio_id: i32,
    asistencia_id: i32,
    entrada: i64,
    salida: Option<i64>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterData {
    sala_id: i32,
    visitante_mat: i32,
    fecha: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterAnswer {
    message: String,
    visitante_mat: i32,
    register_type: String
}

#[tauri::command]
pub async fn get_salas(pool: State<'_, Database>) -> Result<Vec<Sala>, ()> {
    let rows = sqlx::query("SELECT * FROM sala;")
        .fetch_all(&pool.pool)
        .await
        .unwrap();

    let salas: Vec<Sala> = rows
        .iter()
        .map(|x| Sala {
            sala_id: x.get("sala_id"),
            sala_piso: x.get("sala_piso"),
            sala_nom: x.get("sala_nom"),
            biblio_id: x.get("biblio_id"),
        })
        .collect();

    Ok(salas)
}

#[tauri::command]
pub async fn add_registration(pool: State<'_, Database>, data: RegisterData) -> Result<RegisterAnswer, ()> {
    let added_user = add_user_to_database(pool.pool.clone(), data.visitante_mat).await;

    if added_user {
        println!("Added the user to our database");
    }

    let verify = sqlx::query("SELECT * FROM asistencia WHERE visitant_mat = $1 AND salida IS NULL")
        .bind(data.visitante_mat)
        .fetch_optional(&pool.pool)
        .await
        .unwrap();


    if verify.is_none() {
        let row = sqlx::query("insert into asistencia (sala_id, visitant_mat, biblio_id, entrada) values ($1, $2, $3, $4)")
        .bind(data.sala_id)
        .bind(data.visitante_mat)
        .bind(1)
        .bind(data.fecha)
        .execute(&pool.pool)
        .await
        .unwrap();

    let mut answer:RegisterAnswer = RegisterAnswer {
        message: "".to_string(),
        visitante_mat: data.visitante_mat,
        register_type: "Entrada".to_string()
    };

    if row.rows_affected() > 0 {
        answer.message = "Success".to_string();
        Ok(answer)
    } else {
        answer.message = "Failure".to_string();
        Ok(answer)
    }
} else {
    let verify_id:i32 = verify.unwrap().get("asistencia_id");
    let row = sqlx::query("update asistencia set salida = $1 where asistencia_id = $2")
        .bind(data.fecha)
        .bind(verify_id)
        .execute(&pool.pool)
        .await
        .unwrap();

        let mut answer:RegisterAnswer = RegisterAnswer {
            message: "".to_string(),
            visitante_mat: data.visitante_mat,
            register_type: "Salida".to_string()
        };

        if row.rows_affected() > 0 {
            answer.message = "Success".to_string();
            Ok(answer)
        } else {
            answer.message = "Failure".to_string();
            Ok(answer)
        }

    }

}

pub async fn add_user_to_database(pool: PgPool, mat: i32) -> bool {
    let user = sqlx::query("select * from visitant where visitant_mat = $1")
        .bind(mat)
        .fetch_optional(&pool)
        .await
        .unwrap();

    if user.is_none() {
        let add_user = sqlx::query("insert into visitant (visitant_mat) values ($1)")
            .bind(mat)
            .execute(&pool)
            .await
            .unwrap();

        add_user.rows_affected() > 0
    } else {
        false
    }

}

#[tauri::command]
pub async fn get_statistics_by_date(pool: State<'_, Database>, date: i64) -> Result<Vec<Asistencia>, ()> {
    let data = sqlx::query("select * from asistencia where entrada > $1 order by entrada")
        .bind(date)
        .fetch_all(&pool.pool)
        .await
        .unwrap();

    let rows: Vec<Asistencia> = data.iter().map(|x| {
        Asistencia {
            sala_id: x.get("sala_id"),
            visitante_mat: x.get("visitant_mat"),
            biblio_id: x.get("biblio_id"),
            asistencia_id: x.get("asistencia_id"),
            entrada: x.get("entrada"),
            salida: x.get("salida")
        }
    }).collect();

    Ok(rows)
}
