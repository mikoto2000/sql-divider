use std::{env, sync::Arc};

use dotenv::dotenv;

use sqlx::{MySql, Pool, Postgres};

use tauri::Manager;
use tokio::sync::Mutex;

mod command;
mod model;
mod mysql;
mod postgres;
mod sql_parser;

pub struct AppState {
    pub db_type: Arc<Mutex<Option<String>>>,
    pub pg_pool: Arc<Mutex<Option<Pool<Postgres>>>>,
    pub mysql_pool: Arc<Mutex<Option<Pool<MySql>>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    dotenv().ok();

    tauri::Builder::default()
        .setup(move |app| {
            app.manage(AppState {
                db_type: Arc::new(Mutex::new(None)),
                pg_pool: Arc::new(Mutex::new(None)),
                mysql_pool: Arc::new(Mutex::new(None)),
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
            command::open_new_statement_window_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
