pub mod check_constraints;
pub mod key_column_usage;
pub mod table_constraints;

pub use check_constraints::*;
pub use key_column_usage::*;
pub use table_constraints::*;

use crate::postgres::{
    def::*,
    query::{InformationSchema, SchemaQueryBuilder},
};
use sea_query::{Expr, Iden, Order, Query, SelectStatement};
use std::rc::Rc;

/// High level query that combines information from table_constraints and key_column_usage to find
/// Unique constraints for a table or column
pub struct UniqueQueryResult {
    pub constraint_name: String,
    pub table_name: String,
    pub column_name: String,
    pub ordinal_postion: i32,
    pub position_in_unique_constraint: Option<i32>,
}

impl SchemaQueryBuilder {
    pub fn query_unique_constraints(
        &self,
        schema: Rc<dyn Iden>,
        table: Rc<dyn Iden>,
        column: Option<Rc<dyn Iden>>,
    ) -> SelectStatement {
        type Schema = InformationSchema;
        type KcuField = KeyColumnUsageFields;
        type TcField = TableConstraintsField;

        let mut query = Query::select()
        	.columns(vec![
            	(Schema::KeyColumnUsage, KcuField::ConstraintName),
            	(Schema::KeyColumnUsage, KcuField::TableName),
            	(Schema::KeyColumnUsage, KcuField::ColumnName),
            	(Schema::KeyColumnUsage, KcuField::OrdinalPosition),
            	(Schema::KeyColumnUsage, KcuField::PositionInUniqueConstraint),
            ])
            .from((Schema::Schema, Schema::KeyColumnUsage))
            .inner_join(
                (Schema::Schema, Schema::TableConstraints),
                Expr::tbl(Schema::KeyColumnUsage, KcuField::ConstraintName)
                	.equals(Schema::TableConstraints, TcField::ConstraintName)
            )
            .and_where(
            	Expr::tbl(Schema::KeyColumnUsage, KcuField::TableSchema).eq(schema.to_string())
            )
            .and_where(
                Expr::tbl(Schema::KeyColumnUsage, KcuField::TableName).eq(table.to_string())
            );

		if let Some(column) = column {
			query = query.and_where(
				Expr::tbl(Schema::KeyColumnUsage, KcuField::ColumnName).eq(column.to_string())
			);
		}

        query
        	.and_where(
				Expr::tbl(Schema::TableConstraints, TcField::ConstraintType).eq("UNIQUE")
        	)
        	.take()
    }
}

#[cfg(feature = "sqlx-postgres")]
impl From<&PgRow> for UniqueQueryResult {
	fn from(row: &PgRow) -> Self {
		Self {
			constraint_name: row.get(0),
			table_name: row.get(1),
			column_name: row.get(2),
			ordinal_postion: row.get(3),
			position_in_unique_constraint: row.get(4),
		}
	}
}
