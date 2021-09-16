use crate::sqlx_types::{mysql::MySqlRow, MySqlPool};
use sea_query::{MysqlQueryBuilder, SelectStatement};

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

        panic!("This is a mock Executor");
    }
}
