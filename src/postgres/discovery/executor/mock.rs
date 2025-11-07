use crate::{
    Connection,
    sqlx_types::{PgPool, SqlxRow},
};
use sea_query::{PostgresQueryBuilder, SelectStatement};

use crate::{debug_print, sqlx_types::SqlxError};

#[allow(dead_code)]
pub struct Executor {}

pub trait IntoExecutor {
    fn into_executor(self) -> Executor;
}

impl IntoExecutor for PgPool {
    fn into_executor(self) -> Executor {
        Executor {}
    }
}

#[async_trait::async_trait]
impl Connection for Executor {
    async fn query_all(&self, select: SelectStatement) -> Result<Vec<SqlxRow>, SqlxError> {
        let (_sql, _values) = select.build(PostgresQueryBuilder);
        debug_print!("{}, {:?}", _sql, _values);

        panic!("This is a mock Executor");
    }

    async fn query_all_raw(&self, _sql: String) -> Result<Vec<SqlxRow>, SqlxError> {
        debug_print!("{}", _sql);

        panic!("This is a mock Executor");
    }
}
