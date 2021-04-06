#[cfg(feature="sqlx-mysql")]
use sqlx::{Row, mysql::MySqlRow};
use sea_query::{Func, Query, SelectStatement};
use super::SchemaQueryBuilder;

#[derive(sea_query::Iden)]
enum MysqlFunc {
    Version,
}

#[derive(Debug)]
pub struct VersionQueryResult {
    pub version: String,
}

impl SchemaQueryBuilder {
    pub fn query_version(&self) -> SelectStatement {
        Query::select()
            .expr(Func::cust(MysqlFunc::Version).into_simple_expr())
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
