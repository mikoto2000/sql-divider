use std::{env, sync::Arc};

use dotenv::dotenv;

use sqlx::{Pool, Postgres};

use tauri::Manager;
use tokio::sync::Mutex;

mod command;
mod database;
mod model;
mod sql_parser;

pub struct AppState {
    pub pool: Arc<Mutex<Option<Pool<Postgres>>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    dotenv().ok();

    tauri::Builder::default()
        .setup(move |app| {
            app.manage(AppState {
                pool: Arc::new(Mutex::new(None)),
            });
            Ok(())
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            command::connect_command,
            command::close_command,
            command::query_command,
            command::find_select_statement_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
