use std::{collections::HashMap, env, sync::Arc};

use dotenv::dotenv;

use sqlx::{Pool, Postgres};

use tauri::{Manager, State};
use tokio::sync::Mutex;

mod database;

struct AppState {
    pub pool: Arc<Mutex<Pool<Postgres>>>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn query_command(
    state: State<'_, AppState>,
    query: String,
) -> Result<Vec<HashMap<String, String>>, ()> {
    println!("query_command!");
    let state = state.clone();
    let pool = state.pool.clone();

    let result = database::query(&pool, query).await;

    Ok(result)
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
        .invoke_handler(tauri::generate_handler![query_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
