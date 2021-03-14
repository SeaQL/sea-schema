use super::Type;

#[derive(Debug, sea_query::Iden)]
pub enum ColumnFields {
    ColumnName,
    ColumnType,
    IsNullable,
    ColumnKey,
    ColumnDefault,
    Extra,
    ColumnComment,
    TableSchema,
    TableName,
    OrdinalPosition,
}

#[derive(Debug)]
pub struct ColumnInfo {
    pub name: String,
    pub col_type: ColumnType,
    pub key: ColumnKey,
    pub default: Option<ColumnDefault>,
    pub extra: ColumnExtra,
    pub comment: String,
}

pub type ColumnType = Type;

#[derive(Debug)]
pub enum ColumnKey {
    Null,
    Primary,
    Unique,
    Multiple,
}

#[derive(Debug)]
pub struct ColumnDefault {
    pub expr: String,
}

#[derive(Debug, Default, PartialEq)]
pub struct ColumnExtra {
    pub auto_increment: bool,
    pub on_update_current_timestamp: bool,
    pub generated: bool,
    pub default_generated: bool,
}