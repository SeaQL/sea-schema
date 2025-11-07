use super::{InformationSchema, SchemaQueryBuilder};
use crate::sqlx_types::SqlxRow;
use sea_query::{DynIden, Expr, ExprTrait, Order, Query, SelectStatement, Value};

#[derive(Debug, sea_query::Iden)]
/// Ref: https://dev.mysql.com/doc/refman/8.0/en/information-schema-columns-table.html
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

#[derive(Debug, Default)]
pub struct ColumnQueryResult {
    pub column_name: String,
    pub column_type: String,
    pub is_nullable: String,
    pub column_key: String,
    pub column_default: Option<String>,
    pub extra: String,
    pub generation_expression: Option<String>,
    pub column_comment: String,
}

impl SchemaQueryBuilder {
    pub fn query_columns(&self, schema: DynIden, table: DynIden) -> SelectStatement {
        Query::select()
            .columns([
                ColumnFields::ColumnName,
                ColumnFields::ColumnType,
                ColumnFields::IsNullable,
                ColumnFields::ColumnKey,
                ColumnFields::ColumnDefault,
                ColumnFields::Extra,
            ])
            .conditions(
                self.system.is_mysql() && self.system.version >= 50700,
                |q| {
                    q.column(ColumnFields::GenerationExpression);
                },
                |q| {
                    q.expr(Expr::val(Value::String(None)));
                },
            )
            .column(ColumnFields::ColumnComment)
            .from((InformationSchema::Schema, InformationSchema::Columns))
            .and_where(Expr::col(ColumnFields::TableSchema).eq(schema.to_string()))
            .and_where(Expr::col(ColumnFields::TableName).eq(table.to_string()))
            .order_by(ColumnFields::OrdinalPosition, Order::Asc)
            .take()
    }
}

#[cfg(feature = "sqlx-mysql")]
impl From<SqlxRow> for ColumnQueryResult {
    fn from(row: SqlxRow) -> Self {
        use crate::mysql::discovery::GetMySqlValue;
        let row = row.mysql();
        Self {
            column_name: row.get_string(0),
            column_type: row.get_string(1),
            is_nullable: row.get_string(2),
            column_key: row.get_string(3),
            column_default: row.get_string_opt(4),
            extra: row.get_string(5),
            generation_expression: row.get_string_opt(6),
            column_comment: row.get_string(7),
        }
    }
}

#[cfg(not(feature = "sqlx-mysql"))]
impl From<SqlxRow> for ColumnQueryResult {
    fn from(_: SqlxRow) -> Self {
        Self::default()
    }
}
