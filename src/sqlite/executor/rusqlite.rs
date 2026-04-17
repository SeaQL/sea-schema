use crate::{Connection, debug_print, rusqlite_types::RusqliteRow};
use rusqlite::{Connection as RusqliteConnection, Error as RusqliteError};
use sea_query::{SelectStatement, SqliteQueryBuilder};
use sea_query_rusqlite::{RusqliteBinder, rusqlite};

pub struct Executor {
    conn: RusqliteConnection,
}

pub trait IntoExecutor {
    fn into_executor(self) -> Executor;
}

impl IntoExecutor for RusqliteConnection {
    fn into_executor(self) -> Executor {
        Executor { conn: self }
    }
}

#[async_trait::async_trait(?Send)]
impl Connection for Executor {
    fn query_all(&self, select: SelectStatement) -> Result<Vec<RusqliteRow>, RusqliteError> {
        let (sql, values) = select.build_rusqlite(SqliteQueryBuilder);
        debug_print!("{}, {:?}", sql, values);

        let mut stmt = self.conn.prepare(sql.as_str())?;
        let mut rows = stmt.query(&*values.as_params())?;
        let mut res = Vec::new();

        while let Some(row) = rows.next()? {
            res.push(RusqliteRow::from_row(row));
        }

        Ok(res)
    }

    fn query_all_raw(&self, sql: String) -> Result<Vec<RusqliteRow>, RusqliteError> {
        debug_print!("{}", sql);

        let mut stmt = self.conn.prepare(sql.as_str())?;
        let mut rows = stmt.query(())?;
        let mut res = Vec::new();

        while let Some(row) = rows.next()? {
            res.push(RusqliteRow::from_row(row));
        }

        Ok(res)
    }
}
