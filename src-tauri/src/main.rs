// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlx::{PgPool, Postgres};
use sqlx_migrator::migrator::{Migrator, Info, Migrate};

mod attendency;
mod db;
mod migrations;

pub struct Database {
    pool: PgPool,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Could not load environment variables");

    let pool = db::make_pool().await;

    let mut migrator: Migrator<Postgres> = Migrator::default();
    migrator.add_migrations(migrations::migrations());

    migrator.apply_all(&pool).await.expect("Could not run migrations");
    println!("Migrations successfully applied");

        tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![greet, attendency::controller::get_salas,
            attendency::controller::add_registration,
            attendency::controller::get_statistics_by_date])
            .manage(Database{ pool })
            .run(tauri::generate_context!())
            .unwrap();
}
