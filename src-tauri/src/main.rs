// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlx::PgPool;
use tokio::sync::Mutex;
use tauri_plugin_log::LogTarget;

mod attendency;
mod db;
mod migrations;
mod events;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {

    //Basically generates a mutex skeleton of the database pool that will be later overrided by the
    //db::make_database function.
    let placeholder_database: Mutex<Option<PgPool>> = Mutex::new(None);

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().targets([
            LogTarget::LogDir,
            LogTarget::Stdout,
            LogTarget::Webview,
        ]).build())
        .manage(placeholder_database)
        .invoke_handler(tauri::generate_handler![
            greet,
            db::make_database,
            db::run_migrations,
            db::revert_migrations,
            attendency::controller::get_salas,
            attendency::controller::add_registration,
            attendency::controller::get_statistics_by_date,
            events::controller::get_events,
            events::controller::delete_event,
            events::controller::add_event,
            events::controller::get_event_by_id,
            events::controller::add_invitado,
            events::controller::add_event_registration
        ])
        .run(tauri::generate_context!())
        .unwrap();
}
