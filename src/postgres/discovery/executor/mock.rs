use crate::sqlx_types::{postgres::PgRow, PgPool};
use sea_query::{PostgresQueryBuilder, SelectStatement};

use crate::debug_print;

#[allow(dead_code)]
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

impl Executor {
    pub async fn fetch_all(&self, select: SelectStatement) -> Vec<PgRow> {
        let (_sql, _values) = select.build(PostgresQueryBuilder);
        debug_print!("{}, {:?}", _sql, _values);

        panic!("This is a mock Executor");
    }
}
