pub mod check_constraints;
pub mod key_column_usage;
pub mod table_constraints;

pub use check_constraints::*;
pub use key_column_usage::*;
pub use table_constraints::*;

use super::{InformationSchema, SchemaQueryBuilder};
use crate::sqlx_types::SqlxRow;
use sea_query::{Condition, DynIden, Expr, ExprTrait, JoinType, Order, Query, SelectStatement};

#[derive(Debug, Default)]
pub struct TableConstraintsQueryResult {
    // From table_constraints
    pub constraint_schema: String,
    pub constraint_name: String,
    pub table_schema: String,
    pub table_name: String,
    pub constraint_type: String,
    pub is_deferrable: String,
    pub initially_deferred: String,

    // From check_constraints
    pub check_clause: Option<String>,

    // From key_column_usage
    pub column_name: Option<String>,
    pub ordinal_position: Option<i32>,
}

impl SchemaQueryBuilder {
    pub fn query_table_constraints(&self, schema: DynIden, table: DynIden) -> SelectStatement {
        type Schema = InformationSchema;
        type Tcf = TableConstraintsField;
        type Cf = CheckConstraintsFields;
        type Kcuf = KeyColumnUsageFields;
        Query::select()
            .columns(vec![
                (Schema::TableConstraints, Tcf::ConstraintSchema),
                (Schema::TableConstraints, Tcf::ConstraintName),
                (Schema::TableConstraints, Tcf::TableSchema),
                (Schema::TableConstraints, Tcf::TableName),
                (Schema::TableConstraints, Tcf::ConstraintType),
                (Schema::TableConstraints, Tcf::IsDeferrable),
                (Schema::TableConstraints, Tcf::InitiallyDeferred),
            ])
            .column((Schema::CheckConstraints, Cf::CheckClause))
            .columns(vec![
                (Schema::KeyColumnUsage, Kcuf::ColumnName),
                (Schema::KeyColumnUsage, Kcuf::OrdinalPosition),
            ])
            .from((Schema::Schema, InformationSchema::TableConstraints))
            .join(
                JoinType::LeftJoin,
                (Schema::Schema, Schema::CheckConstraints),
                Condition::all()
                    .add(
                        Expr::col((Schema::TableConstraints, Tcf::ConstraintName))
                            .equals((Schema::CheckConstraints, Cf::ConstraintName)),
                    )
                    .add(
                        Expr::col((Schema::TableConstraints, Tcf::ConstraintCatalog))
                            .equals((Schema::CheckConstraints, Cf::ConstraintCatalog)),
                    )
                    .add(
                        Expr::col((Schema::TableConstraints, Tcf::ConstraintSchema))
                            .equals((Schema::CheckConstraints, Cf::ConstraintSchema)),
                    ),
            )
            .join(
                JoinType::LeftJoin,
                (Schema::Schema, Schema::KeyColumnUsage),
                Condition::all()
                    .add(
                        Expr::col((Schema::TableConstraints, Tcf::ConstraintName))
                            .equals((Schema::KeyColumnUsage, Kcuf::ConstraintName)),
                    )
                    .add(
                        Expr::col((Schema::TableConstraints, Tcf::ConstraintCatalog))
                            .equals((Schema::KeyColumnUsage, Kcuf::ConstraintCatalog)),
                    )
                    .add(
                        Expr::col((Schema::TableConstraints, Tcf::ConstraintSchema))
                            .equals((Schema::KeyColumnUsage, Kcuf::ConstraintSchema)),
                    )
                    .add(
                        Expr::col((Schema::TableConstraints, Tcf::TableCatalog))
                            .equals((Schema::KeyColumnUsage, Kcuf::TableCatalog)),
                    )
                    .add(
                        Expr::col((Schema::TableConstraints, Tcf::TableSchema))
                            .equals((Schema::KeyColumnUsage, Kcuf::TableSchema)),
                    )
                    .add(
                        Expr::col((Schema::TableConstraints, Tcf::TableName))
                            .equals((Schema::KeyColumnUsage, Kcuf::TableName)),
                    ),
            )
            .and_where(
                Expr::col((Schema::TableConstraints, Tcf::TableSchema)).eq(schema.to_string()),
            )
            .and_where(Expr::col((Schema::TableConstraints, Tcf::TableName)).eq(table.to_string()))
            .order_by((Schema::TableConstraints, Tcf::ConstraintName), Order::Asc)
            .order_by((Schema::KeyColumnUsage, Kcuf::OrdinalPosition), Order::Asc)
            .take()
    }
}

#[cfg(feature = "sqlx-postgres")]
impl From<SqlxRow> for TableConstraintsQueryResult {
    fn from(row: SqlxRow) -> Self {
        use crate::sqlx_types::Row;
        let row = row.postgres();
        Self {
            constraint_schema: row.get(0),
            constraint_name: row.get(1),
            table_schema: row.get(2),
            table_name: row.get(3),
            constraint_type: row.get(4),
            is_deferrable: row.get(5),
            initially_deferred: row.get(6),

            check_clause: row.get(7),

            column_name: row.get(8),
            ordinal_position: row.get(9),
        }
    }
}

#[cfg(not(feature = "sqlx-postgres"))]
impl From<SqlxRow> for TableConstraintsQueryResult {
    fn from(_: SqlxRow) -> Self {
        Self::default()
    }
}
