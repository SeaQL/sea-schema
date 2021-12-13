use crate::sqlx_types::{sqlite::SqliteRow, SqlitePool};
use sea_query::{SelectStatement, SqliteQueryBuilder};

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

        panic!("This is a mock Executor");
    }

    pub async fn fetch_one(&self, select: SelectStatement) -> SqliteRow {
        let (sql, values) = select.build(SqliteQueryBuilder);
        debug_print!("{}, {:?}", sql, values);

        panic!("This is a mock Executor");
    }

    pub async fn fetch_all_raw(&self, sql: String) -> Vec<SqliteRow> {
        debug_print!("{}", sql);

        panic!("This is a mock Executor");
    }
}
