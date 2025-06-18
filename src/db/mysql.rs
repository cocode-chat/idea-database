use fnv::FnvHashMap;
use serde_json::Value;
use super::Database;
use sqlx::{mysql::{MySqlColumn, MySqlRow, MySqlPool}, Column, Row, TypeInfo, types::Decimal, Error};
use crate::common::model::ColumnMeta;

pub struct MySqlImpl;

impl Database for MySqlImpl {
    type Column = MySqlColumn;
    type Row = MySqlRow;
    type Pool = MySqlPool;

    async fn connect(url: &str) -> Result<Self::Pool, Error> {
        MySqlPool::connect(url).await
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
