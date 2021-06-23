pub mod check_constraints;
pub mod key_column_usage;
pub mod table_constraints;

pub use check_constraints::*;
pub use key_column_usage::*;
pub use table_constraints::*;

use crate::{
    postgres::query::{InformationSchema, SchemaQueryBuilder},
    sqlx_types::postgres::PgRow,
};
use sea_query::{Expr, Iden, Order, Query, SelectStatement};

use std::rc::Rc;

/// High level query result that combines information from table_constraints and key_column_usage
/// to find the name of a unique constraint that applies to a table
pub struct UniqueConstraintNameResult {
    pub schema: String,
    pub constraint_name: String,
}

/// High level query that combines information from table_constraints and key_column_usage to find
/// Unique constraints for a table or column -- each result is one member of the overall constraint
pub struct UniqueQueryResult {
    pub constraint_name: String,
    pub table_name: String,
    pub column_name: String,
    pub ordinal_postion: i32,
    pub position_in_unique_constraint: Option<i32>,
}

impl SchemaQueryBuilder {
    pub fn query_unique_constraint_name(
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
            ])
            .from((Schema::Schema, Schema::KeyColumnUsage))
            .inner_join(
                (Schema::Schema, Schema::TableConstraints),
                Expr::tbl(Schema::KeyColumnUsage, KcuField::ConstraintName)
                    .equals(Schema::TableConstraints, TcField::ConstraintName),
            )
            .and_where(
                Expr::tbl(Schema::KeyColumnUsage, KcuField::TableSchema).eq(schema.to_string()),
            )
            .and_where(Expr::tbl(Schema::KeyColumnUsage, KcuField::TableName).eq(table.to_string()))
            .take();

        if let Some(column) = column {
            let _ = query.and_where(
                Expr::tbl(Schema::KeyColumnUsage, KcuField::ColumnName).eq(column.to_string()),
            );
        }

        query
            .and_where(Expr::tbl(Schema::TableConstraints, TcField::ConstraintType).eq("UNIQUE"))
            .take()
    }

    pub fn query_unique_constraint(schema: Rc<dyn Iden>, name: Rc<dyn Iden>) -> SelectStatement {
        type Schema = InformationSchema;
        type KcuField = KeyColumnUsageFields;
        type TcField = TableConstraintsField;

        Query::select()
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
                    .equals(Schema::TableConstraints, TcField::ConstraintName),
            )
            .and_where(
                Expr::tbl(Schema::KeyColumnUsage, KcuField::TableSchema).eq(schema.to_string()),
            )
            .and_where(
                Expr::tbl(Schema::KeyColumnUsage, KcuField::ConstraintName).eq(name.to_string()),
            )
            .and_where(Expr::tbl(Schema::TableConstraints, TcField::ConstraintType).eq("UNIQUE"))
            .order_by(KcuField::OrdinalPosition, Order::Asc)
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

#[cfg(not(feature = "sqlx-postgres"))]
impl From<&PgRow> for UniqueQueryResult {
    fn from(row: &PgRow) -> Self {
        Self {
            constraint_name: String::default(),
            table_name: String::default(),
            column_name: String::default(),
            ordinal_postion: 1,
            position_in_unique_constraint: None,
        }
    }
}
