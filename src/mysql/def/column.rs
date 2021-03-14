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
pub struct Column {
    name: String,
    col_type: ColumnType,
    key: ColumnKey,
    default: ColumnDefault,
    extra: ColumnExtra,
    comment: String,
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
    expr: String,
}

#[derive(Debug)]
pub struct ColumnExtra {
    auto_increment: bool,
    on_update_current_timestamp: bool,
    generated: bool,
    default_generated: bool,
}