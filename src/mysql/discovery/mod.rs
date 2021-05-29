//! To query & parse MySQL's INFORMATION_SCHEMA and construct a [`Schema`]

use crate::debug_print;
use crate::mysql::def::*;
use crate::mysql::parser::{parse_constraint_query_results, parse_index_query_results};
use crate::mysql::query::{
    ColumnQueryResult, ConstraintQueryResult, IndexQueryResult, SchemaQueryBuilder,
    TableQueryResult, VersionQueryResult,
};
use futures::future;
use sea_query::{Alias, Iden, IntoIden};
use std::rc::Rc;

mod executor;
pub use executor::*;

pub struct SchemaDiscovery {
    pub query: SchemaQueryBuilder,
    pub executor: Executor,
    pub schema: Rc<dyn Iden>,
}

impl SchemaDiscovery {
    pub fn new<E>(executor: E, schema: &str) -> Self
    where
        E: IntoExecutor,
    {
        Self {
            query: SchemaQueryBuilder::default(),
            executor: executor.into_executor(),
            schema: Alias::new(schema).into_iden(),
        }
    }

    pub async fn discover(mut self) -> Schema {
        self.query = SchemaQueryBuilder::new(self.discover_system().await);
        let tables = self.discover_tables().await;
        let tables = future::join_all(
            tables
                .into_iter()
                .map(|t| (&self, t))
                .map(Self::discover_table_static),
        )
        .await;

        Schema {
            schema: self.schema.to_string(),
            system: self.query.system,
            tables,
        }
    }

    pub async fn discover_system(&mut self) -> SystemInfo {
        let rows = self.executor.fetch_all(self.query.query_version()).await;

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

    pub async fn discover_tables(&mut self) -> Vec<TableInfo> {
        let rows = self
            .executor
            .fetch_all(self.query.query_tables(self.schema.clone()))
            .await;

        let tables: Vec<TableInfo> = rows
            .iter()
            .map(|row| {
                let result: TableQueryResult = row.into();
                debug_print!("{:?}", result);
                let table = result.parse();
                debug_print!("{:?}", table);
                table
            })
            .collect();

        debug_print!();
        tables
    }

    async fn discover_table_static(params: (&Self, TableInfo)) -> TableDef {
        let this = params.0;
        let info = params.1;
        Self::discover_table(this, info).await
    }

    pub async fn discover_table(&self, info: TableInfo) -> TableDef {
        let table = Rc::new(Alias::new(info.name.as_str()));
        let columns = self
            .discover_columns(self.schema.clone(), table.clone())
            .await;
        let indexes = self
            .discover_indexes(self.schema.clone(), table.clone())
            .await;
        let foreign_keys = self
            .discover_foreign_keys(self.schema.clone(), table.clone())
            .await;

        TableDef {
            info,
            columns,
            indexes,
            foreign_keys,
        }
    }

    pub async fn discover_columns(
        &self,
        schema: Rc<dyn Iden>,
        table: Rc<dyn Iden>,
    ) -> Vec<ColumnInfo> {
        let rows = self
            .executor
            .fetch_all(self.query.query_columns(schema.clone(), table.clone()))
            .await;

        let columns = rows
            .iter()
            .map(|row| {
                let result: ColumnQueryResult = row.into();
                debug_print!("{:?}", result);
                let column = result.parse();
                debug_print!("{:?}", column);
                return column;
            })
            .collect::<Vec<_>>();

        debug_print!();
        columns
    }

    pub async fn discover_indexes(
        &self,
        schema: Rc<dyn Iden>,
        table: Rc<dyn Iden>,
    ) -> Vec<IndexInfo> {
        let rows = self
            .executor
            .fetch_all(self.query.query_indexes(schema.clone(), table.clone()))
            .await;

        let results: Vec<IndexQueryResult> = rows
            .iter()
            .map(|row| {
                let result = row.into();
                debug_print!("{:?}", result);
                return result;
            })
            .collect();
        debug_print!();

        let indexes = parse_index_query_results(Box::new(results.into_iter()))
            .map(|index| {
                debug_print!("{:?}", index);
                index
            })
            .collect::<Vec<_>>();
        debug_print!();

        indexes
    }

    pub async fn discover_foreign_keys(
        &self,
        schema: Rc<dyn Iden>,
        table: Rc<dyn Iden>,
    ) -> Vec<ForeignKeyInfo> {
        let rows = self
            .executor
            .fetch_all(self.query.query_constraints(schema.clone(), table.clone()))
            .await;

        let results: Vec<ConstraintQueryResult> = rows
            .iter()
            .map(|row| {
                let result = row.into();
                debug_print!("{:?}", result);
                return result;
            })
            .collect();
        debug_print!();

        let foreign_keys = parse_constraint_query_results(Box::new(results.into_iter()))
            .map(|index| {
                debug_print!("{:?}", index);
                index
            })
            .collect::<Vec<_>>();
        debug_print!();

        foreign_keys
    }
}
