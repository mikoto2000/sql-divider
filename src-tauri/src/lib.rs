use std::{collections::HashMap, env, sync::Arc};

use dotenv::dotenv;

use sqlx::Column;
use sqlx::Row;
use sqlx::TypeInfo;
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
    let pool = pool.lock().await;

    let query_result = sqlx::query(&query).fetch_all(&*pool).await.unwrap();
    drop(pool);

    let mut result: Vec<HashMap<String, String>> = vec![];
    for row in query_result {
        let mut map: HashMap<String, String> = HashMap::new();

        for column in row.columns() {
            let type_info = column.type_info();
            let type_name = type_info.name();
            match type_name {
                "INT4" => {
                    let value: i32 = row.try_get(column.ordinal()).unwrap();

                    map.insert(column.name().to_string(), value.to_string());
                    print!("{}, ", value);
                }
                "VARCHAR" => {
                    let value: String = row.try_get(column.ordinal()).unwrap();

                    map.insert(column.name().to_string(), value.to_string());
                    print!("{}, ", value);
                }
                _ => {
                    print!("unknown type {}", type_name);
                }
            }
        }
        result.push(map);
        println!()
    }

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
