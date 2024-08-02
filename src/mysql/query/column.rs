use super::{InformationSchema, SchemaQueryBuilder};
use crate::sqlx_types::mysql::MySqlRow;
use sea_query::{Expr, Iden, Order, Query, SeaRc, SelectStatement, Value};

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
    pub fn query_columns(
        &self,
        schema: SeaRc<dyn Iden>,
        table: SeaRc<dyn Iden>,
    ) -> SelectStatement {
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
impl From<&MySqlRow> for ColumnQueryResult {
    fn from(row: &MySqlRow) -> Self {
        use crate::sqlx_types::Row;
        Self {
            column_name: row.get(0),
            column_type: String::from_utf8(row.get::<Vec<u8>, _>(1)).unwrap(),
            is_nullable: row.get(2),
            column_key: String::from_utf8(row.get::<Vec<u8>, _>(3)).unwrap(),
            column_default: row
                .get::<Option<Vec<u8>>, _>(4)
                .map(|v| String::from_utf8(v).unwrap()),
            extra: row.get(5),
            generation_expression: row
                .get::<Option<Vec<u8>>, _>(6)
                .map(|v| String::from_utf8(v).unwrap()),
            column_comment: String::from_utf8(row.get::<Vec<u8>, _>(7)).unwrap(),
        }
    }
}

#[cfg(not(feature = "sqlx-mysql"))]
impl From<&MySqlRow> for ColumnQueryResult {
    fn from(_: &MySqlRow) -> Self {
        Self::default()
    }
}
