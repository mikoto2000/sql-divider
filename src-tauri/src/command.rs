use std::collections::HashMap;

use tauri::webview::WebviewWindowBuilder;
use tauri::{AppHandle, Emitter, Listener, State};

use crate::{
    model::{Column, Parameter},
    mysql, postgres, sql_parser, AppState,
};

#[tauri::command]
pub async fn connect_command(
    state: State<'_, AppState>,
    db_type: String,
    url: String,
    db: String,
    user: String,
    password: String,
) -> Result<(), String> {
    println!("connect_command!");

    if db_type == String::from("postgres") {
        let pool = postgres::create_postgres_connection_pool(url, db, user, password).await?;

        *state.db_type.lock().await = Some(String::from("postgres"));
        *state.pg_pool.lock().await = Some(pool);
    } else if db_type == String::from("mysql") {
        let pool = mysql::create_mysql_connection_pool(url, db, user, password).await?;

        *state.db_type.lock().await = Some(String::from("mysql"));
        *state.mysql_pool.lock().await = Some(pool);
    } else {
        return Err(String::from("Unknown db_type."));
    }

    Ok(())
}

#[tauri::command]
pub async fn close_command(state: State<'_, AppState>) -> Result<(), String> {
    println!("close_command!");
    let state = state.clone();
    let db_type = state.db_type.lock().await.clone();

    if let Some(db_type) = &db_type {
        if db_type == &String::from("postgres") {
            let pool = state.pg_pool.clone();

            postgres::close_postgres_connection_pool(pool).await?;

            *state.pg_pool.lock().await = None;
        } else if db_type == &String::from("mysql") {
            let pool = state.mysql_pool.clone();

            mysql::close_mysql_connection_pool(pool).await?;

            *state.mysql_pool.lock().await = None;
        } else {
            return Err(String::from("Unknown db_type."));
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn query_command(
    state: State<'_, AppState>,
    query: String,
) -> Result<(Vec<Column>, Vec<HashMap<String, String>>), String> {
    println!("query_command!");
    let state = state.clone();
    let db_type = state.db_type.lock().await.clone();

    if let Some(db_type) = &db_type {
        if db_type == &String::from("postgres") {
            let state = state.clone();
            let pool = state.pg_pool.clone();

            let result = postgres::query_to_postgres(&pool, query).await;

            let result = match result {
                Ok(r) => r,
                Err(e) => return Err(e.to_string()),
            };
            return Ok(result);
        } else if db_type == &String::from("mysql") {
            let state = state.clone();
            let pool = state.mysql_pool.clone();

            let result = mysql::query_to_mysql(&pool, query).await;

            let result = match result {
                Ok(r) => r,
                Err(e) => return Err(e.to_string()),
            };
            return Ok(result);
        } else {
            return Err(String::from("Unknown database type"));
        }
    }

    Err(String::from("Unknown database type"))
}

#[tauri::command]
pub async fn find_select_statement_command(
    state: State<'_, AppState>,
    query: String,
) -> Result<(Vec<String>, Vec<String>), String> {
    println!("find_select_statement_command!");

    let state = state.clone();
    let db_type = state.db_type.lock().await.clone();

    let select_statements = if db_type == Some(String::from("postgres")) {
        sql_parser::find_postgres_select_statement(&query).await
    } else if db_type == Some(String::from("mysql")) {
        sql_parser::find_mysql_select_statement(&query).await
    } else {
        return Err(String::from("Unknown db_type."));
    };

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
