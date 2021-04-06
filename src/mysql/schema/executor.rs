use sqlx::{MySqlConnection, MySqlPool, mysql::MySqlRow};
use sea_query::{MysqlQueryBuilder, SelectStatement};

sea_query::sea_query_driver_mysql!();
use sea_query_driver_mysql::bind_query;

pub enum Executor {
    Pool(MySqlPool),
    Connection(MySqlConnection),
}

pub trait IntoExecutor {
    fn into_executor(self) -> Executor;
}

impl IntoExecutor for MySqlPool {
    fn into_executor(self) -> Executor {
        Executor::Pool(self)
    }
}

impl IntoExecutor for MySqlConnection {
    fn into_executor(self) -> Executor {
        Executor::Connection(self)
    }
}

impl Executor {
    pub async fn fetch_all(&mut self, select: SelectStatement) -> Vec<MySqlRow> {
        let (sql, values) = select.build(MysqlQueryBuilder);
        let query = bind_query(sqlx::query(&sql), &values);
        match self {
            Self::Pool(pool) => query.fetch_all(pool as &MySqlPool).await.unwrap(),
            Self::Connection(conn) => query.fetch_all(conn).await.unwrap(),
        }
    }
}
