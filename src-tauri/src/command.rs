use std::collections::HashMap;

use tauri::State;

use crate::{database, model::Column, sql_parser, AppState};

#[tauri::command]
pub async fn query_command(
    state: State<'_, AppState>,
    query: String,
) -> Result<(Vec<Column>, Vec<HashMap<String, String>>), String> {
    println!("query_command!");
    let state = state.clone();
    let pool = state.pool.clone();

    let result = database::query(&pool, query).await;

    let result = match result {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    Ok(result)
}

#[tauri::command]
pub async fn find_select_statement_command(
    query: String,
) -> Result<Vec<String>, String> {
    println!("find_select_statement_command!");
    let select_statements = sql_parser::find_select_statement(&query).await;

    let result = match select_statements {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    Ok(result)
}

