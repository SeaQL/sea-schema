pub use sqlx::*;

pub type RusqliteError = sqlx::Error;

#[cfg(not(feature = "sqlx-mysql"))]
pub struct MySqlPool;

#[cfg(not(feature = "sqlx-postgres"))]
pub struct PgPool;

#[cfg(all(not(feature = "rusqlite"), not(feature = "sqlx-sqlite")))]
pub struct RusqliteConnection;

pub enum RusqliteRow {
    #[cfg(feature = "sqlx-mysql")]
    MySql(sqlx::mysql::MySqlRow),
    #[cfg(feature = "sqlx-postgres")]
    Postgres(sqlx::postgres::PgRow),
    #[cfg(feature = "rusqlite")]
    Sqlite(sqlx::sqlite::SqliteRow),
}

#[allow(unreachable_patterns)]
impl RusqliteRow {
    #[cfg(feature = "sqlx-mysql")]
    pub fn mysql(self) -> mysql::MySqlRow {
        match self {
            Self::MySql(row) => row,
            _ => panic!("Not MySql"),
        }
    }

    #[cfg(feature = "sqlx-postgres")]
    pub fn postgres(self) -> postgres::PgRow {
        match self {
            Self::Postgres(row) => row,
            _ => panic!("Not Postgres"),
        }
    }

    #[cfg(feature = "rusqlite")]
    pub fn sqlite(self) -> sqlite::SqliteRow {
        match self {
            Self::Sqlite(row) => row,
            _ => panic!("Not Sqlite"),
        }
    }
}

#[cfg(feature = "rusqlite")]
pub fn connect_sqlite(s: &str) -> Result<RusqliteConnection, RusqliteError> {
    RusqliteConnection::connect(s)
}

#[cfg(feature = "rusqlite")]
pub fn execute_sqlite(pool: &RusqliteConnection, sql: &str) -> Result<(), RusqliteError> {
    sqlx::query(sqlx::AssertSqlSafe(sql)).execute(pool)?;
    Ok(())
}
