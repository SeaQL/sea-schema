use sea_query::{MysqlQueryBuilder, SelectStatement};
use crate::sqlx_types::{MySqlPool, mysql::MySqlRow};

use crate::debug_print;

pub struct Executor {
    pool: MySqlPool,
}

pub trait IntoExecutor {
    fn into_executor(self) -> Executor;
}

impl IntoExecutor for MySqlPool {
    fn into_executor(self) -> Executor {
        Executor {
            pool: self
        }
    }
}

impl Executor {
    pub async fn fetch_all(&self, select: SelectStatement) -> Vec<MySqlRow> {
        let (sql, values) = select.build(MysqlQueryBuilder);
        debug_print!("{}, {:?}", sql, values);
        debug_print!();

        Vec::new()
    }
}
