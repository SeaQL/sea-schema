use sea_query::{MysqlQueryBuilder, SelectStatement};
use sqlx::{mysql::MySqlRow, MySqlPool};

sea_query::sea_query_driver_mysql!();
use sea_query_driver_mysql::bind_query;

use crate::debug_print;

pub struct Executor {
    pool: MySqlPool,
}

pub trait IntoExecutor {
    fn into_executor(self) -> Executor;
}

impl IntoExecutor for MySqlPool {
    fn into_executor(self) -> Executor {
        Executor { pool: self }
    }
}

impl Executor {
    pub async fn fetch_all(&self, select: SelectStatement) -> Vec<MySqlRow> {
        let (sql, values) = select.build(MysqlQueryBuilder);
        debug_print!("{}, {:?}", sql, values);

        let query = bind_query(sqlx::query(&sql), &values);
        query
            .fetch_all(&mut self.pool.acquire().await.unwrap())
            .await
            .unwrap()
    }
}
