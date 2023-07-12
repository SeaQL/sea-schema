use sea_query::{SelectStatement, SqliteQueryBuilder};
use sea_query_binder::SqlxBinder;
use sqlx::{sqlite::SqliteRow, SqlitePool};
use std::ops::DerefMut;

use crate::{debug_print, sqlx_types::SqlxError};

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
    pub async fn fetch_all(&self, select: SelectStatement) -> Result<Vec<SqliteRow>, SqlxError> {
        let (sql, values) = select.build_sqlx(SqliteQueryBuilder);
        debug_print!("{}, {:?}", sql, values);

        sqlx::query_with(&sql, values)
            .fetch_all(self.pool.acquire().await?.deref_mut())
            .await
    }

    pub async fn fetch_one(&self, select: SelectStatement) -> Result<SqliteRow, SqlxError> {
        let (sql, values) = select.build_sqlx(SqliteQueryBuilder);
        debug_print!("{}, {:?}", sql, values);

        sqlx::query_with(&sql, values)
            .fetch_one(self.pool.acquire().await?.deref_mut())
            .await
    }

    pub async fn fetch_all_raw(&self, sql: String) -> Result<Vec<SqliteRow>, SqlxError> {
        debug_print!("{}", sql);

        sqlx::query(&sql)
            .fetch_all(self.pool.acquire().await?.deref_mut())
            .await
    }
}
