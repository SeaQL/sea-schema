use crate::mysql::def::*;

#[cfg(feature="sqlx-mysql")]
mod executor;

pub struct Schema {
    pub tables: Vec<Table>,
}

pub struct Table {
    pub info: TableInfo,
    pub columns: Vec<ColumnInfo>,
    pub indexes: Vec<IndexInfo>,
    pub foreign_keys: Vec<ForeignKeyInfo>,
}