use crate::mysql::def::*;

use sqlx::{Executor, MySql};
use sea_query::{Alias, Iden, MysqlQueryBuilder};
use crate::mysql::query::{SchemaQuery, ColumnQueryResult, ConstraintQueryResult, IndexQueryResult, VersionQueryResult};
use crate::debug_print;
use super::*;

sea_query::sea_query_driver_mysql!();
use sea_query_driver_mysql::bind_query;

impl Schema {
    pub async fn discover<'a, E>(exec: E) where E: Executor<'a, Database = MySql> {
        let schema_query = SchemaQuery::new(Self::discover_version(exec).await);
    }

    pub async fn discover_version<'a, E>(exec: E) -> Version
        where E: Executor<'a, Database = MySql> {
        let schema_query = SchemaQuery::default();

        let (sql, values) = schema_query.query_version().build(MysqlQueryBuilder);
        debug_print!("{}", sql);
        debug_print!();

        let rows = 
            bind_query(sqlx::query(&sql), &values)
                .fetch_all(exec)
                .await
                .unwrap();

        for row in rows.iter() {
            let result: VersionQueryResult = row.into();
            debug_print!("{:?}", result);
            let version = result.parse();
            debug_print!("{:?}", version);
            debug_print!();
            return version;
        }
        panic!("failed to discover version")
    }
}
