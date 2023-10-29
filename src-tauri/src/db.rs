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
    db_pass: String
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
    let new_pool = PgPool::connect(&conn_str).await.expect("Could not connect to database");

    //Updates tauri State pool to be used across the application.
    *state.lock().await = Some(new_pool);

    println!("Successfully connected to database.");
    Ok(true)
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
