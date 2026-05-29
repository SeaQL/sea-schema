use sea_query::{PostgresQueryBuilder, SelectStatement};
use sea_query_sqlx::SqlxBinder;
use sqlx::PgPool;

use crate::{
    Connection, debug_print,
    rusqlite_types::{RusqliteError, RusqliteRow},
};

pub struct Executor {
    pool: PgPool,
}

pub trait IntoExecutor {
    fn into_executor(self) -> Executor;
}

impl IntoExecutor for PgPool {
    fn into_executor(self) -> Executor {
        Executor { pool: self }
    }
}

impl Connection for Executor {
    fn query_all(&self, select: SelectStatement) -> Result<Vec<RusqliteRow>, RusqliteError> {
        let (sql, values) = select.build_sqlx(PostgresQueryBuilder);
        debug_print!("{}, {:?}", sql, values);

        Ok(sqlx::query_with(sqlx::AssertSqlSafe(sql), values)
            .fetch_all(&mut *self.pool.acquire()?)?
            .into_iter()
            .map(RusqliteRow::Postgres)
            .collect())
    }

    fn query_all_raw(&self, sql: String) -> Result<Vec<RusqliteRow>, RusqliteError> {
        debug_print!("{}", sql);

        Ok(sqlx::query(sqlx::AssertSqlSafe(sql))
            .fetch_all(&mut *self.pool.acquire()?)?
            .into_iter()
            .map(RusqliteRow::Postgres)
            .collect())
    }
}
