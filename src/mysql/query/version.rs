use super::SchemaQueryBuilder;
use crate::sqlx_types::SqlxRow;
use sea_query::{Func, Query, SelectStatement};

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
        Query::select().expr(Func::cust(MysqlFunc::Version)).take()
    }
}

#[cfg(feature = "sqlx-mysql")]
impl From<SqlxRow> for VersionQueryResult {
    fn from(row: SqlxRow) -> Self {
        use crate::mysql::discovery::GetMySqlValue;
        let row = row.mysql();
        Self {
            version: row.get_string(0),
        }
    }
}

#[cfg(not(feature = "sqlx-mysql"))]
impl From<SqlxRow> for VersionQueryResult {
    fn from(_: SqlxRow) -> Self {
        Self::default()
    }
}
