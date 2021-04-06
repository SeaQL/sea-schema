use crate::mysql::def::*;

mod executor;
mod discovery;

pub use executor::*;
pub use discovery::*;

pub struct Schema {
    pub system: SystemInfo,
    pub tables: Vec<TableDef>,
}

pub struct TableDef {
    pub info: TableInfo,
    pub columns: Vec<ColumnInfo>,
    pub indexes: Vec<IndexInfo>,
    pub foreign_keys: Vec<ForeignKeyInfo>,
}