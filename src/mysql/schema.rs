use crate::mysql::def::*;

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
