pub mod check_constraints;
pub mod key_column_usage;
pub mod referential_constraints;
pub mod table_constraints;

pub use check_constraints::*;
pub use key_column_usage::*;
pub use referential_constraints::*;
pub use table_constraints::*;

use super::{InformationSchema, SchemaQueryBuilder};
use crate::sqlx_types::{postgres::PgRow, Row};
use sea_query::{Alias, Expr, Iden, JoinType, Order, Query, SelectStatement};
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct TableConstraintsQueriesResult {
    // From table_constraints
    constraint_schema: String,
    constraint_name: String,
    table_schema: String,
    table_name: String,
    constraint_type: String,
    is_deferrable: String,
    initially_deferred: String,

    // From check_constraints
    check_clause: Option<String>,

    // From key_column_usage
    column_name: Option<String>,
    ordinal_position: Option<i32>,
    position_in_unique_constraint: Option<i32>,

    // From referential_constraints
    unique_constraint_schema: Option<String>,
    unique_constraint_name: Option<String>,
    match_option: Option<String>,
    update_rule: Option<String>,
    delete_rule: Option<String>,

    // From key_column_usage as part of subquery involving referential_constraints
    referential_key_table_name: Option<String>,
    referential_key_column_name: Option<String>,
    referential_key_ordinal_pos: Option<i32>,
}

impl SchemaQueryBuilder {
    pub fn query_table_constriants(schema: Rc<dyn Iden>, table: Rc<dyn Iden>) -> SelectStatement {
        type Schema = InformationSchema;
        type Tcf = TableConstraintsField;
        type Cf = CheckConstraintsFields;
        type Kcuf = KeyColumnUsageFields;
        type RefC = ReferentialConstraintsFields;

        let rcsq = Alias::new("referential_constraints_subquery");

        Query::select()
            .columns(vec![
                (Schema::TableConstraints, Tcf::ConstraintSchema),
                (Schema::TableConstraints, Tcf::ConstraintName),
                (Schema::TableConstraints, Tcf::ConstraintType),
                (Schema::TableConstraints, Tcf::IsDeferrable),
                (Schema::TableConstraints, Tcf::InitiallyDeferred),
            ])
            .column((Schema::CheckConstraints, Cf::CheckClause))
            .columns(vec![
                (Schema::KeyColumnUsage, Kcuf::ColumnName),
                (Schema::KeyColumnUsage, Kcuf::OrdinalPosition),
                (Schema::KeyColumnUsage, Kcuf::PositionInUniqueConstraint),
            ])
            .columns(vec![
                (rcsq.clone(), RefC::UniqueConstraintSchema),
                (rcsq.clone(), RefC::UniqueConstraintName),
                (rcsq.clone(), RefC::MatchOption),
                (rcsq.clone(), RefC::UpdateRule),
                (rcsq.clone(), RefC::DeleteRule),
            ])
            .columns(vec![
                (rcsq.clone(), Kcuf::TableName),
                (rcsq.clone(), Kcuf::ColumnName),
                (rcsq.clone(), Kcuf::OrdinalPosition),
            ])
            .from((Schema::Schema, InformationSchema::TableConstraints))
            .join(
                JoinType::LeftJoin,
                (Schema::Schema, Schema::CheckConstraints),
                Expr::tbl(Schema::TableConstraints, Tcf::ConstraintName)
                    .equals(Schema::CheckConstraints, Cf::ConstraintName),
            )
            .join(
                JoinType::LeftJoin,
                (Schema::Schema, Schema::KeyColumnUsage),
                Expr::tbl(Schema::TableConstraints, Tcf::ConstraintName)
                    .equals(Schema::KeyColumnUsage, Kcuf::ConstraintName),
            )
            .join_subquery(
                JoinType::LeftJoin,
                Query::select()
                    .columns(vec![
                        (Schema::ReferentialConstraints, RefC::ConstraintName),
                        (Schema::ReferentialConstraints, RefC::UniqueConstraintSchema),
                        (Schema::ReferentialConstraints, RefC::UniqueConstraintName),
                        (Schema::ReferentialConstraints, RefC::MatchOption),
                        (Schema::ReferentialConstraints, RefC::UpdateRule),
                        (Schema::ReferentialConstraints, RefC::DeleteRule),
                    ])
                    .columns(vec![
                        (Schema::KeyColumnUsage, Kcuf::TableName),
                        (Schema::KeyColumnUsage, Kcuf::ColumnName),
                        (Schema::KeyColumnUsage, Kcuf::OrdinalPosition),
                    ])
                    .from((Schema::Schema, Schema::ReferentialConstraints))
                    .left_join(
                        (Schema::Schema, Schema::KeyColumnUsage),
                        Expr::tbl(Schema::ReferentialConstraints, RefC::UniqueConstraintName)
                            .equals(Schema::KeyColumnUsage, Kcuf::ConstraintName),
                    )
                    .take(),
                rcsq.clone(),
                Expr::tbl(Schema::TableConstraints, Tcf::ConstraintName)
                    .equals(rcsq.clone(), RefC::ConstraintName),
            )
            .and_where(
                Expr::col((Schema::TableConstraints, Tcf::TableSchema)).eq(schema.to_string()),
            )
            .and_where(Expr::col((Schema::TableConstraints, Tcf::TableName)).eq(table.to_string()))
            .order_by((Schema::TableConstraints, Tcf::ConstraintName), Order::Asc)
            .order_by((Schema::KeyColumnUsage, Kcuf::OrdinalPosition), Order::Asc)
            .order_by((rcsq.clone(), RefC::UniqueConstraintName), Order::Asc)
            .order_by((rcsq.clone(), Tcf::ConstraintName), Order::Asc)
            .take()
    }
}

#[cfg(feature = "sqlx-postres")]
impl From<&PgRow> for TableConstraintsQueriesResult {
    fn from(row: &PgRow) -> Self {
        Self {
            constraint_schema: row.get(0),
            constraint_name: row.get(1),
            constraint_type: row.get(2),
            is_deferrable: row.get(3),
            initially_deferred: row.get(4),
        }
    }
}

#[cfg(not(feature = "sqlx-postres"))]
impl From<&PgRow> for TableConstraintsQueriesResult {
    fn from(_row: &PgRow) -> Self {
        Self::default()
    }
}
