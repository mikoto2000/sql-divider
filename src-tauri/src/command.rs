use std::collections::HashMap;

use tauri::webview::WebviewWindowBuilder;
use tauri::{AppHandle, Emitter, Listener, State};

use crate::{
    database,
    model::{Column, Parameter},
    sql_parser, AppState,
};

#[tauri::command]
pub async fn connect_command(
    state: State<'_, AppState>,
    url: String,
    db: String,
    user: String,
    password: String,
) -> Result<(), String> {
    println!("connect_command!");

    let pool = database::create_connection_pool(url, db, user, password).await?;

    *state.pool.lock().await = Some(pool);

    Ok(())
}

#[tauri::command]
pub async fn close_command(state: State<'_, AppState>) -> Result<(), String> {
    println!("close_command!");

    let state = state.clone();
    let pool = state.pool.clone();

    database::close_connection_pool(pool).await?;

    *state.pool.lock().await = None;

    Ok(())
}

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
pub async fn find_select_statement_command(query: String) -> Result<Vec<String>, String> {
    println!("find_select_statement_command!");
    let select_statements = sql_parser::find_select_statement(&query).await;

    let result = match select_statements {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    Ok(result)
}

#[tauri::command]
pub async fn open_new_statement_window_command(
    app: AppHandle,
    parameter_pattern: String,
    parameters: Vec<Parameter>,
    select_statements: Vec<String>,
    columns: Vec<Column>,
    query_result: Vec<HashMap<String, String>>,
) -> Result<(), tauri::Error> {
    println!("open_new_statement_window_command!");

    let md5 = md5::compute(select_statements[0].clone());
    let window_label = format!("select_{:x}", md5);

    let builder = WebviewWindowBuilder::new(
        &app,
        &window_label,
        tauri::WebviewUrl::App("statement.html".into()),
    );

    let new_webview = builder.title(select_statements[0].clone()).build()?;

    new_webview.once("done", move |_| {
        app.emit_to(
            window_label,
            "data",
            (
                parameter_pattern,
                parameters,
                select_statements,
                columns,
                query_result,
            ),
        )
        .expect("emit_to error.");
    });

    new_webview.show()?;

    Ok(())
}
