#[cfg(feature="sqlx-mysql")]
use sqlx::{Row, mysql::MySqlRow};
use sea_query::{Func, Query, SimpleExpr, SelectStatement};
use super::SchemaQuery;

#[derive(sea_query::Iden)]
enum MysqlFunc {
    Version,
}

#[derive(Debug)]
pub struct VersionQueryResult {
    pub version: String,
}

impl SchemaQuery {
    pub fn query_version() -> SelectStatement {
        Query::select()
            .expr(Into::<SimpleExpr>::into(Func::cust(MysqlFunc::Version)))
            .take()
    }
}

#[cfg(feature="sqlx-mysql")]
impl From<&MySqlRow> for VersionQueryResult {
    fn from(row: &MySqlRow) -> Self {
        Self {
            version: row.get(0),
        }
    }
}
