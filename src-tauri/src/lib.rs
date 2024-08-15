use std::{env, sync::Arc};

use dotenv::dotenv;

use sqlx::{Pool, Postgres};

use tauri::Manager;
use tokio::sync::Mutex;

mod command;
mod database;

struct AppState {
    pub pool: Arc<Mutex<Pool<Postgres>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    dotenv().ok();

    let pool = database::create_connection_pool().await;

    tauri::Builder::default()
        .setup(move |app| {
            app.manage(AppState {
                pool: Arc::new(Mutex::new(pool)),
            });
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![command::query_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
