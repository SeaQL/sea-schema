//! To query & parse MySQL's INFORMATION_SCHEMA and construct a [`Schema`]

use crate::debug_print;
use crate::mysql::def::*;
use crate::mysql::parser::{parse_foreign_key_query_results, parse_index_query_results};
use crate::mysql::query::{
    ColumnQueryResult, ForeignKeyQueryResult, IndexQueryResult, SchemaQueryBuilder,
    TableQueryResult, VersionQueryResult,
};
use crate::{
    Connection,
    sqlx_types::{MySqlPool, SqlxError},
};
use sea_query::{Alias, DynIden, IntoIden, SeaRc};

mod executor;
pub use executor::*;

pub struct SchemaDiscovery<C: Connection> {
    pub query: SchemaQueryBuilder,
    pub schema: DynIden,
    conn: C,
}

impl SchemaDiscovery<Executor> {
    /// Discover schema from a SQLx pool
    pub fn new(pool: MySqlPool, schema: &str) -> Self {
        Self::conn(pool.into_executor(), schema)
    }
}

impl<C: Connection> SchemaDiscovery<C> {
    /// Discover schema from a generic SQLx connection
    pub fn conn(conn: C, schema: &str) -> Self {
        SchemaDiscovery {
            query: SchemaQueryBuilder::default(),
            schema: Alias::new(schema).into_iden(),
            conn,
        }
    }

    pub async fn discover(mut self) -> Result<Schema, SqlxError> {
        self.query = SchemaQueryBuilder::new(self.discover_system().await?);
        let mut tables = Vec::new();

        for table in self.discover_tables().await? {
            tables.push(self.discover_table(table).await?);
        }

        Ok(Schema {
            schema: self.schema.to_string(),
            system: self.query.system,
            tables,
        })
    }

    pub async fn discover_system(&mut self) -> Result<SystemInfo, SqlxError> {
        let rows = self.conn.query_all(self.query.query_version()).await?;

        #[allow(clippy::never_loop)]
        for row in rows {
            let result: VersionQueryResult = row.into();
            debug_print!("{:?}", result);
            let version = result.parse();
            debug_print!("{:?}", version);
            return Ok(version);
        }
        Err(SqlxError::RowNotFound)
    }

    pub async fn discover_tables(&mut self) -> Result<Vec<TableInfo>, SqlxError> {
        let rows = self
            .conn
            .query_all(self.query.query_tables(self.schema.clone()))
            .await?;

        let tables: Vec<TableInfo> = rows
            .into_iter()
            .map(|row| {
                let result: TableQueryResult = row.into();
                debug_print!("{:?}", result);
                let table = result.parse();
                debug_print!("{:?}", table);
                table
            })
            .collect();

        Ok(tables)
    }

    pub async fn discover_table(&self, info: TableInfo) -> Result<TableDef, SqlxError> {
        let table = SeaRc::new(Alias::new(info.name.as_str()));
        let columns = self
            .discover_columns(self.schema.clone(), table.clone(), &self.query.system)
            .await?;
        let indexes = self
            .discover_indexes(self.schema.clone(), table.clone())
            .await?;
        let foreign_keys = self
            .discover_foreign_keys(self.schema.clone(), table.clone())
            .await?;

        Ok(TableDef {
            info,
            columns,
            indexes,
            foreign_keys,
        })
    }

    pub async fn discover_columns(
        &self,
        schema: DynIden,
        table: DynIden,
        system: &SystemInfo,
    ) -> Result<Vec<ColumnInfo>, SqlxError> {
        let rows = self
            .conn
            .query_all(self.query.query_columns(schema.clone(), table.clone()))
            .await?;

        let columns = rows
            .into_iter()
            .map(|row| {
                let result: ColumnQueryResult = row.into();
                debug_print!("{:?}", result);
                let column = result.parse(system);
                debug_print!("{:?}", column);
                column
            })
            .collect::<Vec<_>>();

        Ok(columns)
    }

    pub async fn discover_indexes(
        &self,
        schema: DynIden,
        table: DynIden,
    ) -> Result<Vec<IndexInfo>, SqlxError> {
        let rows = self
            .conn
            .query_all(self.query.query_indexes(schema.clone(), table.clone()))
            .await?;

        let results = rows.into_iter().map(|row| {
            let result: IndexQueryResult = row.into();
            debug_print!("{:?}", result);
            result
        });

        Ok(parse_index_query_results(Box::new(results))
            .inspect(|_index| {
                debug_print!("{:?}", _index);
            })
            .collect())
    }

    pub async fn discover_foreign_keys(
        &self,
        schema: DynIden,
        table: DynIden,
    ) -> Result<Vec<ForeignKeyInfo>, SqlxError> {
        let rows = self
            .conn
            .query_all(self.query.query_foreign_key(schema.clone(), table.clone()))
            .await?;

        let results = rows.into_iter().map(|row| {
            let result: ForeignKeyQueryResult = row.into();
            debug_print!("{:?}", result);
            result
        });

        Ok(parse_foreign_key_query_results(Box::new(results))
            .inspect(|_index| {
                debug_print!("{:?}", _index);
            })
            .collect())
    }
}
