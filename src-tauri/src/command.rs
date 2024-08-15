use std::collections::HashMap;

use tauri::State;

use crate::{database, AppState};

#[tauri::command]
pub async fn query_command(
    state: State<'_, AppState>,
    query: String,
) -> Result<Vec<HashMap<String, String>>, ()> {
    println!("query_command!");
    let state = state.clone();
    let pool = state.pool.clone();

    let result = database::query(&pool, query).await;

    Ok(result)
}

