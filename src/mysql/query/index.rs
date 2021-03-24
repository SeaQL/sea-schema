use std::rc::Rc;
#[cfg(feature="sqlx-mysql")]
use sqlx::{Row, mysql::MySqlRow};
use sea_query::{Expr, Iden, Order, Query, SelectStatement};
use crate::mysql::def::*;
use super::SchemaQuery;

#[derive(Debug)]
pub struct IndexQueryResult {
    pub non_unique: i32,
    pub index_name: String,
    pub column_name: Option<String>,
    pub collation: Option<String>,
    pub sub_part: Option<i32>,
    pub nullable: String,
    pub index_type: String,
    pub index_comment: String,
    pub expression: Option<String>,
}

impl SchemaQuery {
    pub fn query_indexes(schema: Rc<dyn Iden>, table: Rc<dyn Iden>) -> SelectStatement {
        Query::select()
            .columns(vec![
                StatisticsFields::NonUnique,
                StatisticsFields::IndexName,
                StatisticsFields::ColumnName,
                StatisticsFields::Collation,
                StatisticsFields::SubPart,
                StatisticsFields::Nullable,
                StatisticsFields::IndexType,
                StatisticsFields::IndexComment,
                // StatisticsFields::Expression,
            ])
            .from_schema(InformationSchema::Schema, InformationSchema::Statistics)
            .and_where(Expr::col(StatisticsFields::TableSchema).eq(schema.to_string()))
            .and_where(Expr::col(StatisticsFields::TableName).eq(table.to_string()))
            .order_by(StatisticsFields::IndexName, Order::Asc)
            .order_by(StatisticsFields::SeqInIndex, Order::Asc)
            .take()
    }
}

#[cfg(feature="sqlx-mysql")]
impl From<&MySqlRow> for IndexQueryResult {
    fn from(row: &MySqlRow) -> Self {
        Self {
            non_unique: row.get(0),
            index_name: row.get(1),
            column_name: row.get(2),
            collation: row.get(3),
            sub_part: row.get(4),
            nullable: row.get(5),
            index_type: row.get(6),
            index_comment: row.get(7),
            expression: None, // row.get(8),
        }
    }
}
