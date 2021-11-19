use crate::postgres::def::EnumRow;
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

    /// Fetches enums from the enum column. There are many ways to do this however,
    /// this function uses the SQL statement
    /// SELECT pg_type.typname, pg_enum.enumlabel FROM pg_type JOIN pg_enum ON pg_enum.enumtypid = pg_type.oid;
    pub async fn get_enums(&self, select: SelectStatement) -> Vec<EnumRow> {
        let (sql, values) = select.build(PostgresQueryBuilder);
        debug_print!("{}, {:?}", sql, values);

        let query = bind_query(sqlx::query(&sql), &values);

        let rows = query
            .fetch_all(&mut self.pool.acquire().await.unwrap())
            .await
            .unwrap();

        rows.iter()
            .map(|pg_row| {
                let column: EnumRow = pg_row.into();

                column
            })
            .collect::<Vec<EnumRow>>()
    }
}
