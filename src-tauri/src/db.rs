use sqlx_migrator::migrator::{Migrator, Migrate, Info};
use tokio::sync::Mutex;

use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres};
use tauri::State;

use super::migrations;

#[derive(Debug, Serialize, Deserialize)]
pub struct DBCredentials {
    db_host: String,
    db_user: String,
    db_port: i32,
    db_name: String,
    db_pass: String,
    db_table: String
}

#[tauri::command]
pub async fn make_database(credentials:DBCredentials, state: State<'_, Mutex<Option<PgPool>>>) -> Result<bool, ()> {
    //Make a String formatting credentials.
    let conn_str = format!("postgres://{}:{}@{}:{}/{}",
        credentials.db_user,
        credentials.db_pass,
        credentials.db_host,
        credentials.db_port,
        credentials.db_name
    );

    //Use the formatted string to generate postgres pool.
    let new_pool = PgPool::connect(&conn_str).await;

    match new_pool {
        Ok(pool) => {
            //Updates tauri State pool to be used across the application.
            *state.lock().await = Some(pool);
            Ok(true)
        },
        Err(_e) => {
            Ok(false)
        }
    }

}

#[tauri::command]
pub async fn run_migrations(state: State<Mutex<Option<PgPool>>, '_>) -> Result<bool, ()> {
    let guard = state.lock().await;
    let pool = guard.as_ref().unwrap();

    let mut migrator: Migrator<Postgres> = Migrator::default();
    migrator.add_migrations(migrations::migrations());
    migrator.apply_all(pool).await.expect("Could not run migrations");
    println!("Successfully applied all migrations");

    Ok(true)
}

#[tauri::command]
pub async fn revert_migrations(state: State<Mutex<Option<PgPool>>, '_>) -> Result<bool, ()> {
    let guard = state.lock().await;
    let pool = guard.as_ref().unwrap();

    let mut migrator: Migrator<Postgres> = Migrator::default();
    migrator.add_migrations(migrations::migrations());
    migrator.revert_all(pool).await.expect("Could not revert migrations");
    println!("Successfully reverted all migrations");

    Ok(true)
}

#[tauri::command]
pub async fn change_sala_name(state: State<Mutex<Option<PgPool>>, '_>, name: String) -> Result<bool, ()> {
    let guard = state.lock().await;
    let pool = guard.as_ref().unwrap();

    let query = sqlx::query("update sala set sala_nombre = $1 where sala_id = 1")
        .bind(name)
        .execute(pool)
        .await
        .unwrap();

    Ok(query.rows_affected() > 0)
}
