//! To query & parse MySQL's INFORMATION_SCHEMA and construct a [`Schema`]

use crate::debug_print;
use crate::postgres::def::*;
use crate::postgres::query::{
    ColumnQueryResult, SchemaQueryBuilder, TableConstraintsQueryResult, TableQueryResult,
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
            tables,
        }
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
        let (check_constraints, unique_keys, references) = self
            .discover_constraints(self.schema.clone(), table.clone())
            .await;

        TableDef {
            info,
            columns,
            check_constraints,
            unique_keys,
            references,
            of_type: None,
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

    pub async fn discover_constraints(
        &self,
        schema: Rc<dyn Iden>,
        table: Rc<dyn Iden>,
    ) -> (Vec<Check>, Vec<Unique>, Vec<References>) {
        // let rows = self
        //     .executor
        //     .fetch_all(self.query.discover_constraints(schema.clone(), table.clone()))
        //     .await;

        // let constraints: Vec<Constraint> = rows
        //     .iter()
        //     .map(|row| {
        //         let result: TableConstraintsQueryResult = row.into();
        //         debug_print!("{:?}", result);
        //         let constraint = result.parse();
        //         debug_print!("{:?}", result);
        //         constraint
        //     })
        //     .collect();
        // debug_print!();

        (Vec::new(), Vec::new(), Vec::new())
    }
}
