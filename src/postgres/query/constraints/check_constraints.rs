use super::{InformationSchema, SchemaQueryBuilder};
use crate::sqlx_types::{postgres::PgRow, Row};
use sea_query::{Expr, Iden, Query, SelectStatement};
use std::rc::Rc;

#[derive(Debug, sea_query::Iden)]
/// Ref: https://www.postgresql.org/docs/13/infoschema-check-constraints.html
pub enum CheckConstraints {
    ConstraintSchema,
    ConstraintName,
    CheckClause,
}

#[derive(Debug, Default)]
pub struct CheckConstraintQueryResult {
    constraint_schema: String,
    constraint_name: String,
    check_clause: String,
}

impl SchemaQueryBuilder {
    pub fn query_check_constraints(schema: Rc<dyn Iden>, name: Rc<dyn Iden>) -> SelectStatement {
        Query::select()
            .columns(vec![
                CheckConstraints::ConstraintSchema,
                CheckConstraints::ConstraintName,
                CheckConstraints::CheckClause,
            ])
            .from((
                InformationSchema::Schema,
                InformationSchema::CheckConstraints,
            ))
            .and_where(Expr::col(CheckConstraints::ConstraintName).eq(name.to_string()))
            .take()
    }
}

#[cfg(feature = "sqlx-postgres")]
impl From<&PgRow> for CheckConstraintQueryResult {
    fn from(row: &PgRow) -> Self {
        Self {
            constraint_schema: row.get(0),
            constraint_name: row.get(1),
            check_clause: row.get(2),
        }
    }
}

#[cfg(not(feature = "sqlx-postres"))]
impl From<&PgRow> for CheckConstraintQueryResult {
    fn from(row: &PgRow) -> Self {
        Self::default()
    }
}
