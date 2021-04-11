use sea_query::{Func, Query, SelectStatement};
use crate::sqlx_types::{Row, mysql::MySqlRow};
use super::SchemaQueryBuilder;

#[derive(sea_query::Iden)]
enum MysqlFunc {
    Version,
}

#[derive(Debug, Default)]
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

#[cfg(not(feature="sqlx-mysql"))]
impl From<&MySqlRow> for VersionQueryResult {
    fn from(row: &MySqlRow) -> Self {
        Self::default()
    }
}
