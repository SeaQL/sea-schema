pub mod check_constraints;
pub mod key_column_usage;
pub mod referential_constraints;
pub mod table_constraints;

pub use check_constraints::*;
pub use key_column_usage::*;
pub use referential_constraints::*;
pub use table_constraints::*;

use super::{InformationSchema, SchemaQueryBuilder};
use crate::postgres::query::select_base_table_and_view;
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
    pub position_in_unique_constraint: Option<i32>,

    // From referential_constraints
    pub unique_constraint_schema: Option<String>,
    pub unique_constraint_name: Option<String>,
    pub match_option: Option<String>,
    pub update_rule: Option<String>,
    pub delete_rule: Option<String>,

    // From key_column_usage as part of subquery involving referential_constraints
    pub referential_key_table_name: Option<String>,
    pub referential_key_column_name: Option<String>,
}

impl SchemaQueryBuilder {
    pub fn query_table_constraints(&self, schema: DynIden, table: DynIden) -> SelectStatement {
        type Schema = InformationSchema;
        type Tcf = TableConstraintsField;
        type Cf = CheckConstraintsFields;
        type Kcuf = KeyColumnUsageFields;
        type RefC = ReferentialConstraintsFields;

        let rcsq = "referential_constraints_subquery";

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
                (Schema::KeyColumnUsage, Kcuf::PositionInUniqueConstraint),
            ])
            .columns(vec![
                (rcsq, RefC::UniqueConstraintSchema),
                (rcsq, RefC::UniqueConstraintName),
                (rcsq, RefC::MatchOption),
                (rcsq, RefC::UpdateRule),
                (rcsq, RefC::DeleteRule),
            ])
            .columns(vec![(rcsq, Kcuf::TableName), (rcsq, Kcuf::ColumnName)])
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
            .join_subquery(
                JoinType::LeftJoin,
                Query::select()
                    .distinct()
                    .columns(vec![
                        (Schema::ReferentialConstraints, RefC::ConstraintName),
                        (Schema::ReferentialConstraints, RefC::UniqueConstraintSchema),
                        (Schema::ReferentialConstraints, RefC::UniqueConstraintName),
                        (Schema::ReferentialConstraints, RefC::MatchOption),
                        (Schema::ReferentialConstraints, RefC::UpdateRule),
                        (Schema::ReferentialConstraints, RefC::DeleteRule),
                    ])
                    .columns(vec![
                        (Schema::ConstraintColumnUsage, Kcuf::TableName),
                        (Schema::ConstraintColumnUsage, Kcuf::ColumnName),
                    ])
                    .columns(vec![
                        // Extract the ordinal position of the referenced primary keys
                        (Schema::KeyColumnUsage, Kcuf::OrdinalPosition),
                    ])
                    .from((Schema::Schema, Schema::ReferentialConstraints))
                    .left_join(
                        (Schema::Schema, Schema::ConstraintColumnUsage),
                        Expr::col((Schema::ReferentialConstraints, RefC::ConstraintName))
                            .equals((Schema::ConstraintColumnUsage, Kcuf::ConstraintName)),
                    )
                    .left_join(
                        // Join the key_column_usage rows for the referenced primary keys
                        (Schema::Schema, Schema::KeyColumnUsage),
                        Condition::all()
                            .add(
                                Expr::col((Schema::ConstraintColumnUsage, Kcuf::ColumnName))
                                    .equals((Schema::KeyColumnUsage, Kcuf::ColumnName)),
                            )
                            .add(
                                Expr::col((
                                    Schema::ReferentialConstraints,
                                    RefC::UniqueConstraintName,
                                ))
                                .equals((Schema::KeyColumnUsage, Kcuf::ConstraintName)),
                            )
                            .add(
                                Expr::col((
                                    Schema::ReferentialConstraints,
                                    RefC::UniqueConstraintSchema,
                                ))
                                .equals((Schema::KeyColumnUsage, Kcuf::ConstraintSchema)),
                            ),
                    )
                    .and_where(Expr::col((Schema::ReferentialConstraints, RefC::ConstraintSchema)).eq(schema.to_string()))
                    .take(),
                rcsq,
                Condition::all()
                    .add(
                        Expr::col((Schema::TableConstraints, Tcf::ConstraintName))
                            .equals((rcsq, RefC::ConstraintName)),
                    )
                    .add(
                        Condition::any()
                            .add(
                                // Only join when the referenced primary key position matches position_in_unique_constraint for the foreign key column
                                Expr::col((
                                    Schema::KeyColumnUsage,
                                    Kcuf::PositionInUniqueConstraint,
                                ))
                                .equals((rcsq, Kcuf::OrdinalPosition)),
                            )
                            .add(
                                // Allow foreign key column without unique constraint
                                Expr::col((rcsq, Kcuf::OrdinalPosition)).is_null(),
                            ),
                    ),
            )
            .and_where(
                Expr::col((Schema::TableConstraints, Tcf::TableSchema)).eq(schema.to_string()),
            )
            .and_where(Expr::col((Schema::TableConstraints, Tcf::TableName)).eq(table.to_string()))
            .cond_where(
                Condition::any()
                    .add(Expr::col((rcsq, Kcuf::TableName)).is_null())
                    .add(
                        Expr::col((rcsq, Kcuf::TableName))
                            .not_in_subquery(select_base_table_and_view()),
                    ),
            )
            .order_by((Schema::TableConstraints, Tcf::ConstraintName), Order::Asc)
            .order_by((Schema::KeyColumnUsage, Kcuf::OrdinalPosition), Order::Asc)
            .order_by((rcsq, RefC::UniqueConstraintName), Order::Asc)
            .order_by((rcsq, Tcf::ConstraintName), Order::Asc)
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
            position_in_unique_constraint: row.get(10),

            unique_constraint_schema: row.get(11),
            unique_constraint_name: row.get(12),
            match_option: row.get(13),
            update_rule: row.get(14),
            delete_rule: row.get(15),

            referential_key_table_name: row.get(16),
            referential_key_column_name: row.get(17),
        }
    }
}

#[cfg(not(feature = "sqlx-postgres"))]
impl From<SqlxRow> for TableConstraintsQueryResult {
    fn from(_: SqlxRow) -> Self {
        Self::default()
    }
}
