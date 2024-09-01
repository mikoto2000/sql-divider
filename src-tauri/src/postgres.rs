use std::{collections::HashMap, sync::Arc};

use sqlx::types::chrono::NaiveDate;
use sqlx::types::BigDecimal;
use sqlx::Column;
use sqlx::Error;
use sqlx::Row;
use sqlx::TypeInfo;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use tokio::sync::Mutex;

pub async fn create_postgres_connection_pool(
    url: String,
    db: String,
    user: String,
    password: String,
) -> Result<Pool<Postgres>, String> {
    let database_url = format!("postgres://{}:{}@{}/{}", user, password, url, db);

    let result = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await;

    let result = match result {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    Ok(result)
}

pub async fn close_postgres_connection_pool(
    pool: Arc<Mutex<Option<Pool<Postgres>>>>,
) -> Result<(), String> {

    let pool = pool.lock().await;
    pool.clone().unwrap().close().await;
    drop(pool);

    Ok(())
}

pub async fn query_to_postgres(
    pool: &Arc<Mutex<Option<Pool<Postgres>>>>,
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
                    let value: f32 = row.try_get(column.ordinal()).unwrap();

                    map.insert(column.name().to_string(), value.to_string());
                }
                "FLOAT8" => {
                    let value: f64 = row.try_get(column.ordinal()).unwrap();

                    map.insert(column.name().to_string(), value.to_string());
                }
                "NUMERIC" => {
                    let value: BigDecimal = row.try_get(column.ordinal()).unwrap();

                    map.insert(column.name().to_string(), value.to_string());
                }
                "BOOL" => {
                    let value: bool = row.try_get(column.ordinal()).unwrap();

                    map.insert(column.name().to_string(), value.to_string());
                }
                "INT1" => {
                    let value: i8 = row.try_get(column.ordinal()).unwrap();

                    map.insert(column.name().to_string(), value.to_string());
                }
                "INT2" => {
                    let value: i16 = row.try_get(column.ordinal()).unwrap();

                    map.insert(column.name().to_string(), value.to_string());
                }
                "INT4" => {
                    let value: i32 = row.try_get(column.ordinal()).unwrap();

                    map.insert(column.name().to_string(), value.to_string());
                }
                "INT8" => {
                    let value: i64 = row.try_get(column.ordinal()).unwrap();

                    map.insert(column.name().to_string(), value.to_string());
                }
                "CHAR" => {
                    let value: String = row.try_get(column.ordinal()).unwrap();

                    map.insert(column.name().to_string(), value.to_string());
                }
                "VARCHAR" => {
                    let value: String = row.try_get(column.ordinal()).unwrap();

                    map.insert(column.name().to_string(), value.to_string());
                }
                "TEXT" => {
                    let value: String = row.try_get(column.ordinal()).unwrap();

                    map.insert(column.name().to_string(), value.to_string());
                }
                "DATE" => {
                    let value: NaiveDate = row.try_get(column.ordinal()).unwrap();

                    map.insert(column.name().to_string(), value.to_string());
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

