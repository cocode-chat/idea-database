pub mod mysql;
pub mod postgres;


use fnv::FnvHashMap;
use sqlx::{Column, Row};
use crate::common::model::ColumnMeta;

pub trait Database {
    type Column: Column;
    type Row: Row;
    type Pool;

    async fn connect(url: &str) -> Result<Self::Pool, sqlx::Error>;
    async fn load_databases(pool: &Self::Pool) -> Result<Vec<String>, sqlx::Error>;
    async fn load_tables(pool: &Self::Pool, schema: &str) -> Result<Vec<String>, sqlx::Error>;
    async fn load_table_meta(pool: &Self::Pool, schema: &str, table: &str) -> Result<FnvHashMap<String, ColumnMeta>, sqlx::Error>;
    async fn query_one(pool: &Self::Pool, sql: &str, params: Vec<String>) -> Result<Option<FnvHashMap<String, serde_json::Value>>, sqlx::Error>;
    async fn query_list(pool: &Self::Pool, sql: &str, params: Vec<String>) -> Result<Vec<FnvHashMap<String, serde_json::Value>>, sqlx::Error>;
}
