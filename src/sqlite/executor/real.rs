use sea_query::{SelectStatement, SqliteQueryBuilder};
use sea_query_sqlx::SqlxBinder;
use sqlx::SqlitePool;

use crate::{
    Connection, debug_print,
    sqlx_types::{SqlxError, SqlxRow},
};

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

#[async_trait::async_trait]
impl Connection for Executor {
    async fn query_all(&self, select: SelectStatement) -> Result<Vec<SqlxRow>, SqlxError> {
        let (sql, values) = select.build_sqlx(SqliteQueryBuilder);
        debug_print!("{}, {:?}", sql, values);

        Ok(sqlx::query_with(&sql, values)
            .fetch_all(&mut *self.pool.acquire().await?)
            .await?
            .into_iter()
            .map(SqlxRow::Sqlite)
            .collect())
    }

    async fn query_all_raw(&self, sql: String) -> Result<Vec<SqlxRow>, SqlxError> {
        debug_print!("{}", sql);

        Ok(sqlx::query(&sql)
            .fetch_all(&mut *self.pool.acquire().await?)
            .await?
            .into_iter()
            .map(SqlxRow::Sqlite)
            .collect())
    }
}
