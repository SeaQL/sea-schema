#![allow(dead_code)]

pub struct MySqlPool;

pub mod mysql {
    pub struct MySqlRow;
}

pub struct PgPool;

pub mod postgres {
    pub struct PgRow;
}

pub struct SqlitePool;

pub mod sqlite {
    pub struct SqliteRow;
}

pub trait Row {}

#[derive(Debug)]
pub struct Error;

#[derive(Debug)]
pub enum SqlxError {
    RowNotFound,
    PoolClosed,
}

pub enum SqlxRow {
    SqlxMySql(mysql::MySqlRow),
    SqlxPostgres(postgres::PgRow),
    SqlxSqlite(sqlite::SqliteRow),
}

impl SqlxRow {
    pub fn mysql(self) -> mysql::MySqlRow {
        match self {
            Self::SqlxMySql(row) => row,
            _ => panic!("Not SqlxMySql"),
        }
    }

    pub fn postgres(self) -> postgres::PgRow {
        match self {
            Self::SqlxPostgres(row) => row,
            _ => panic!("Not SqlxPostgres"),
        }
    }

    pub fn sqlite(self) -> sqlite::SqliteRow {
        match self {
            Self::SqlxSqlite(row) => row,
            _ => panic!("Not SqlxSqlite"),
        }
    }
}
