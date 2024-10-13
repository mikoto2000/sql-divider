use std::{collections::HashMap, sync::Arc};

use sqlx::mysql::MySqlPoolOptions;
use sqlx::types::chrono::NaiveDate;
use sqlx::types::BigDecimal;
use sqlx::Column;
use sqlx::Error;
use sqlx::MySql;
use sqlx::Pool;
use sqlx::Row;
use sqlx::TypeInfo;

use tokio::sync::Mutex;

pub async fn create_mysql_connection_pool(
    url: String,
    db: String,
    user: String,
    password: String,
) -> Result<Pool<MySql>, String> {
    let database_url = format!("mysql://{}:{}@{}/{}", user, password, url, db);

    let result = MySqlPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await;

    let result = match result {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    Ok(result)
}

pub async fn close_mysql_connection_pool(
    pool: Arc<Mutex<Option<Pool<MySql>>>>,
) -> Result<(), String> {

    let pool = pool.lock().await;
    pool.clone().unwrap().close().await;
    drop(pool);

    Ok(())
}

pub async fn query_to_mysql(
    pool: &Arc<Mutex<Option<Pool<MySql>>>>,
    query: String,
) -> Result<(Vec<crate::model::Column>, Vec<HashMap<String, String>>), Error> {
    let pool = pool.lock().await;
    let pool = pool.clone().unwrap();
    let query_result = sqlx::query(&query).fetch_all(&pool).await?;
    drop(pool);

    let mut result: Vec<HashMap<String, String>> = vec![];
    for row in &query_result {
        let mut map: HashMap<String, String> = HashMap::new();

        for column in row.columns() {
            let type_info = column.type_info();
            let type_name = type_info.name();
            match type_name {
                "FLOAT4" => {
                    let value: Result<f32, _> = row.try_get(column.ordinal());

                    if value.is_ok() {
                        map.insert(column.name().to_string(), value.unwrap().to_string());
                    } else {
                        map.insert(column.name().to_string(), "NULL".to_string());
                    }
                }
                "FLOAT8" => {
                    let value: Result<f64, _> = row.try_get(column.ordinal());

                    if value.is_ok() {
                        map.insert(column.name().to_string(), value.unwrap().to_string());
                    } else {
                        map.insert(column.name().to_string(), "NULL".to_string());
                    }
                }
                "DECIMAL" => {
                    let value: Result<BigDecimal, _> = row.try_get(column.ordinal());

                    if value.is_ok() {
                        map.insert(column.name().to_string(), value.unwrap().to_string());
                    } else {
                        map.insert(column.name().to_string(), "NULL".to_string());
                    }
                }
                "BOOL" => {
                    let value: Result<bool, _> = row.try_get(column.ordinal());

                    if value.is_ok() {
                        map.insert(column.name().to_string(), value.unwrap().to_string());
                    } else {
                        map.insert(column.name().to_string(), "NULL".to_string());
                    }
                }
                "INT1" => {
                    let value: Result<i8, _> = row.try_get(column.ordinal());

                    if value.is_ok() {
                        map.insert(column.name().to_string(), value.unwrap().to_string());
                    } else {
                        map.insert(column.name().to_string(), "NULL".to_string());
                    }
                }
                "SMALLINT" => {
                    let value: Result<i16, _> = row.try_get(column.ordinal());

                    if value.is_ok() {
                        map.insert(column.name().to_string(), value.unwrap().to_string());
                    } else {
                        map.insert(column.name().to_string(), "NULL".to_string());
                    }
                }
                "INT" => {
                    let value: Result<i32, _> = row.try_get(column.ordinal());

                    if value.is_ok() {
                        map.insert(column.name().to_string(), value.unwrap().to_string());
                    } else {
                        map.insert(column.name().to_string(), "NULL".to_string());
                    }
                }
                "BIGINT" => {
                    let value: Result<i64, _> = row.try_get(column.ordinal());

                    if value.is_ok() {
                        map.insert(column.name().to_string(), value.unwrap().to_string());
                    } else {
                        map.insert(column.name().to_string(), "NULL".to_string());
                    }
                }
                "CHAR" => {
                    let value: Result<String, _> = row.try_get(column.ordinal());

                    if value.is_ok() {
                        map.insert(column.name().to_string(), value.unwrap().to_string());
                    } else {
                        map.insert(column.name().to_string(), "NULL".to_string());
                    }
                }
                "VARCHAR" => {
                    let value: Result<String, _> = row.try_get(column.ordinal());

                    if value.is_ok() {
                        map.insert(column.name().to_string(), value.unwrap().to_string());
                    } else {
                        map.insert(column.name().to_string(), "NULL".to_string());
                    }
                }
                "TEXT" => {
                    let value: Result<String, _> = row.try_get(column.ordinal());

                    if value.is_ok() {
                        map.insert(column.name().to_string(), value.unwrap().to_string());
                    } else {
                        map.insert(column.name().to_string(), "NULL".to_string());
                    }
                }
                "DATE" => {
                    let value: Result<NaiveDate, _> = row.try_get(column.ordinal());

                    if value.is_ok() {
                        map.insert(column.name().to_string(), value.unwrap().to_string());
                    } else {
                        map.insert(column.name().to_string(), "NULL".to_string());
                    }
                }
                _ => {
                    println!("{}", type_name);
                }
            }
        }
        result.push(map);
    }

    let mut columns: Vec<crate::model::Column> = vec![];
    if query_result.len() > 0 {
        for column in query_result[0].columns() {
            columns.push(crate::model::Column {
                ordinal: column.ordinal(),
                name: column.name().to_string(),
            });
        }
    }

    Ok((columns, result))
}

