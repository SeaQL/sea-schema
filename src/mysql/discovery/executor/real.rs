use sea_query::{MysqlQueryBuilder, SelectStatement};
use sea_query_sqlx::SqlxBinder;
use sqlx::MySqlPool;

use crate::{
    Connection, debug_print,
    sqlx_types::{Row, SqlxError, SqlxRow, mysql::MySqlRow},
};

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

pub trait GetMySqlValue {
    fn get_string(&self, idx: usize) -> String;

    fn get_string_opt(&self, idx: usize) -> Option<String>;
}

impl GetMySqlValue for MySqlRow {
    fn get_string(&self, idx: usize) -> String {
        String::from_utf8(self.get::<Vec<u8>, _>(idx)).unwrap()
    }

    fn get_string_opt(&self, idx: usize) -> Option<String> {
        self.get::<Option<Vec<u8>>, _>(idx)
            .map(|v| String::from_utf8(v).unwrap())
    }
}

#[async_trait::async_trait]
impl Connection for Executor {
    async fn query_all(&self, select: SelectStatement) -> Result<Vec<SqlxRow>, SqlxError> {
        let (sql, values) = select.build_sqlx(MysqlQueryBuilder);
        debug_print!("{}, {:?}", sql, values);

        Ok(sqlx::query_with(&sql, values)
            .fetch_all(&mut *self.pool.acquire().await?)
            .await?
            .into_iter()
            .map(SqlxRow::MySql)
            .collect())
    }

    async fn query_all_raw(&self, sql: String) -> Result<Vec<SqlxRow>, SqlxError> {
        debug_print!("{}", sql);

        Ok(sqlx::query(&sql)
            .fetch_all(&mut *self.pool.acquire().await?)
            .await?
            .into_iter()
            .map(SqlxRow::MySql)
            .collect())
    }
}
