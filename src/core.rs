use crate::db::Database;

pub struct DBConn<D: Database> {
    pool: D::Pool,
}

impl<D: Database> DBConn<D> {
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let pool = D::connect(url)?;
        let mut ds = Self { pool };
        ds.init().await?;
        Ok(ds)
    }

    async fn init(&mut self) -> Result<(), sqlx::Error> {
        let db_list = D::load_databases(&self.pool).await?;
        // 更新全局缓存...
        for db_name in db_list {
            D::load_tables(&self.pool, &db_name).await?;
        }
        Ok(())
    }
}


// #[cfg(feature = "mysql")]
// type CurrentDb = mysql::MySqlImpl;
//
// #[cfg(feature = "postgres")]
// type CurrentDb = postgres::PostgresImpl;
//
// async fn main() {
//     let conn = DBConn::<CurrentDb>::new("your_connection_string").await.unwrap();
//     let result = conn.query_list("SELECT * FROM users", vec![]).await.unwrap();
// }