use sea_query::{SelectStatement, SqliteQueryBuilder};
use sqlx::{sqlite::SqliteRow, SqlitePool};

sea_query::sea_query_driver_sqlite!();
use sea_query_driver_sqlite::bind_query;

use crate::debug_print;

pub struct Executor {
    pool: SqlitePool,
}

pub trait IntoExecutor {
    fn into_executor(self) -> Executor;
}

impl IntoExecutor for SqlitePool {
    fn into_executor(self) -> Executor {
        Executor { pool: self }
    }
}

impl Executor {
    pub async fn fetch_all(&self, select: SelectStatement) -> Vec<SqliteRow> {
        let (sql, values) = select.build(SqliteQueryBuilder);
        debug_print!("{}, {:?}", sql, values);

        let query = bind_query(sqlx::query(&sql), &values);
        query
            .fetch_all(&mut self.pool.acquire().await.unwrap())
            .await
            .unwrap()
    }

    pub async fn fetch_one(&self, select: SelectStatement) -> SqliteRow {
        let (sql, values) = select.build(SqliteQueryBuilder);
        debug_print!("{}, {:?}", sql, values);

        let query = bind_query(sqlx::query(&sql), &values);
        query
            .fetch_one(&mut self.pool.acquire().await.unwrap())
            .await
            .unwrap()
    }

    pub async fn fetch_all_raw(&self, sql: String) -> Vec<SqliteRow> {
        debug_print!("{}", sql);

        sqlx::query(&sql)
            .fetch_all(&mut self.pool.acquire().await.unwrap())
            .await
            .unwrap()
    }
}
