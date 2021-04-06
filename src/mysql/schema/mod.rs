use sea_query::Iden;
use crate::mysql::def::*;

#[cfg(feature="sqlx-mysql")]
mod executor;
#[cfg(feature="sqlx-mysql")]
mod discovery;

#[cfg(feature="sqlx-mysql")]
pub use executor::*;
#[cfg(feature="sqlx-mysql")]
pub use discovery::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Schema {
    pub system: SystemInfo,
    pub tables: Vec<TableDef>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TableDef {
    pub info: TableInfo,
    pub columns: Vec<ColumnInfo>,
    pub indexes: Vec<IndexInfo>,
    pub foreign_keys: Vec<ForeignKeyInfo>,
}
