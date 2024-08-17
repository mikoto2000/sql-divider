use std::{collections::HashMap, sync::Arc};

use sqlx::Column;
use sqlx::Error;
use sqlx::Row;
use sqlx::TypeInfo;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use tokio::sync::Mutex;

pub async fn create_connection_pool(
    url: String,
    db: String,
    user: String,
    password: String,
) -> Pool<Postgres> {
    let database_url = format!("postgres://{}:{}@{}/{}", user, password, url, db);

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap()
}

pub async fn close_connection_pool(
    pool: Arc<Mutex<Pool<Postgres>>>,
) -> Result<(), String> {

    let pool = pool.lock().await;
    pool.close().await;

    Ok(())
}

pub async fn query(
    pool: &Arc<Mutex<Pool<Postgres>>>,
    query: String,
) -> Result<(Vec<crate::model::Column>, Vec<HashMap<String, String>>), Error> {
    let pool = pool.lock().await;
    let query_result = sqlx::query(&query).fetch_all(&*pool).await?;
    drop(pool);

    let mut result: Vec<HashMap<String, String>> = vec![];
    for row in &query_result {
        let mut map: HashMap<String, String> = HashMap::new();

        for column in row.columns() {
            let type_info = column.type_info();
            let type_name = type_info.name();
            match type_name {
                "INT4" => {
                    let value: i32 = row.try_get(column.ordinal()).unwrap();

                    map.insert(column.name().to_string(), value.to_string());
                }
                "VARCHAR" => {
                    let value: String = row.try_get(column.ordinal()).unwrap();

                    map.insert(column.name().to_string(), value.to_string());
                }
                _ => {}
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
