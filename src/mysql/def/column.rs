use super::Type;

#[derive(Debug, sea_query::Iden)]
pub enum ColumnFields {
    TableCatalog,
    TableSchema,
    TableName,
    ColumnName,
    OrdinalPosition,
    ColumnDefault,
    IsNullable,
    DataType,
    CharacterMaximumLength,
    CharacterOctetLength,
    NumericPrecision,
    NumericScale,
    DatetimePrecision,
    CharacterSetName,
    CollationName,
    ColumnType,
    ColumnKey,
    Extra,
    Privileges,
    ColumnComment,
    GenerationExpression,
    SrsId,
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