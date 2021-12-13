use sea_query::{PostgresQueryBuilder, SelectStatement};
use sqlx::{postgres::PgRow, PgPool};

sea_query::sea_query_driver_postgres!();
use sea_query_driver_postgres::bind_query;

use crate::debug_print;

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
        let (sql, values) = select.build(PostgresQueryBuilder);
        debug_print!("{}, {:?}", sql, values);

        let query = bind_query(sqlx::query(&sql), &values);
        query
            .fetch_all(&mut self.pool.acquire().await.unwrap())
            .await
            .unwrap()
    }
}
