pub use sqlx::*;

pub type SqlxError = sqlx::Error;

pub enum SqlxRow {
    #[cfg(feature = "sqlx-mysql")]
    MySql(sqlx::mysql::MySqlRow),
    #[cfg(feature = "sqlx-postgres")]
    Postgres(sqlx::postgres::PgRow),
    #[cfg(feature = "sqlx-sqlite")]
    Sqlite(sqlx::sqlite::SqliteRow),
}

#[allow(unreachable_patterns)]
impl SqlxRow {
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

    #[cfg(feature = "sqlx-sqlite")]
    pub fn sqlite(self) -> sqlite::SqliteRow {
        match self {
            Self::Sqlite(row) => row,
            _ => panic!("Not Sqlite"),
        }
    }
}
