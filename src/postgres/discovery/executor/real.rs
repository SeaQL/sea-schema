use sea_query::{PostgresQueryBuilder, SelectStatement};
use sqlx::{postgres::PgRow, PgPool, Row};

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

    pub async fn get_enums(&self, select: SelectStatement) -> Vec<EnumColumn> {
        let (sql, values) = select.build(PostgresQueryBuilder);
        debug_print!("{}, {:?}", sql, values);

        let query = bind_query(sqlx::query(&sql), &values);

        let rows = query
            .fetch_all(&mut self.pool.acquire().await.unwrap())
            .await
            .unwrap();

        rows.iter()
            .map(|pg_row| {
                let column: EnumColumn = pg_row.into();

                column
            })
            .collect::<Vec<EnumColumn>>()
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct EnumColumn {
    pub name: String,
    pub value: String,
}

// #[cfg(feature = "sqlx-postgres")]
impl From<&PgRow> for EnumColumn {
    fn from(row: &PgRow) -> Self {
        Self {
            name: row.get(0),
            value: row.get(1),
        }
    }
}
