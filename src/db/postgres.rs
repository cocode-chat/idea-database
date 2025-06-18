use fnv::FnvHashMap;
use serde_json::Value;
use super::Database;
use sqlx::{postgres::{PgColumn, PgRow, PgPool}, Row, Column, Error};
use crate::common::model::ColumnMeta;

pub struct PostgresImpl;

impl Database for PostgresImpl {
    type Column = PgColumn;
    type Row = PgRow;
    type Pool = PgPool;

    fn connect(url: &str) -> Result<Self::Pool, Error> {
        todo!()
    }

    async fn load_databases(pool: &Self::Pool) -> Result<Vec<String>, Error> {
        todo!()
    }

    async fn load_tables(pool: &Self::Pool, schema: &str) -> Result<Vec<String>, Error> {
        todo!()
    }

    async fn load_table_meta(pool: &Self::Pool, schema: &str, table: &str) -> Result<FnvHashMap<String, ColumnMeta>, Error> {
        todo!()
    }

    async fn query_one(pool: &Self::Pool, sql: &str, params: Vec<String>) -> Result<Option<FnvHashMap<String, Value>>, Error> {
        todo!()
    }

    async fn query_list(pool: &Self::Pool, sql: &str, params: Vec<String>) -> Result<Vec<FnvHashMap<String, Value>>, Error> {
        todo!()
    }
}
