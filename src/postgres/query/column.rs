use super::{InformationSchema, KeyColumnUsage, SchemaQueryBuilder, TableConstraints};
use crate::sqlx_types::{postgres::PgRow, Row};
use sea_query::{Expr, Iden, Query, SelectStatement};
use std::rc::Rc;

#[derive(Debug, sea_query::Iden)]
/// Ref: https://www.postgresql.org/docs/13/infoschema-columns.html
pub enum ColumnsField {
    TableCatalog,
    TableSchema,
    TableName,
    ColumnName,
    OrdinalPosition,
    ColumnDefault,
    IsNullable,
    DataType,
    CharacterMaximumlength,
    CharacterOctetLength,
    NumericPrecision,
    NumericPrecisionRadix,
    NumericScale,
    DatetimePrecision,
    IntervalType,
    CollationCatalog,
    CollationSchema,
    CollationName,
    DomainCatalog,
    DomainSchema,
    DomainName,
    UdtCatalog,
    UdtSchema,
    UdtName,
    DtdIdentifier,
    IsIdentity,
    IdentityGeneration,
    IdentityStart,
    IdentityIncrement,
    IdentityMaximum,
    IdentityMinimum,
    IdentityCycle,
    IsGenerated,
    GeneratedExpression,
    IsUpdatable,
}

#[derive(Debug, Default)]
pub struct ColumnQueryResult {
    pub column_name: String,
    pub column_type: String,
    pub column_default: Option<String>,
    pub column_generated: Option<String>,
    pub is_nullable: String,

    // Declared or implicit parameters of numeric types; null for other data types
    pub numeric_precision: Option<i32>,
    pub numeric_precision_radix: Option<i32>,
    pub numeric_scale: Option<i32>,
}

impl SchemaQueryBuilder {
    pub fn query_columns(&self, schema: Rc<dyn Iden>, table: Rc<dyn Iden>) -> SelectStatement {
        Query::select()
            .columns(vec![
                (InformationSchema::Columns, ColumnsField::ColumnName),
                (InformationSchema::Columns, ColumnsField::DataType),
                (InformationSchema::Columns, ColumnsField::ColumnDefault),
                (
                    InformationSchema::Columns,
                    ColumnsField::GeneratedExpression,
                ),
                (InformationSchema::Columns, ColumnsField::IsNullable),
            ])
            .from((InformationSchema::Schema, InformationSchema::Columns))
            .and_where(Expr::col(ColumnsField::TableSchema).eq(schema.to_string()))
            .and_where(Expr::col(ColumnsField::TableName).eq(table.to_string()))
            .take()
    }
}

#[cfg(feature = "sqlx-postres")]
impl From<&PgRow> for ColumnQueryResult {
    fn from(row: &PgRow) -> Self {
        Self {
            column_name: row.get(0),
            column_type: row.get(1),
            column_default: row.get(2),
            column_generated: row.get(3),
            is_nullable: row.get(4),
        }
    }
}

#[cfg(not(feature = "sqlx-postres"))]
impl From<&PgRow> for ColumnQueryResult {
    fn from(row: &PgRow) -> Self {
        Self::default()
    }
}
