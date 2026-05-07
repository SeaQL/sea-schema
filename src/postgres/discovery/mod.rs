//! To query & parse MySQL's INFORMATION_SCHEMA and construct a [`Schema`]

use crate::debug_print;
use crate::postgres::def::*;
use crate::postgres::parser::{
    parse_table_constraint_query_results, parse_unique_index_query_results,
};
use crate::postgres::query::{
    ColumnQueryResult, EnumQueryResult, SchemaQueryBuilder, SearchPathResult,
    TableConstraintsQueryResult, TableQueryResult, UniqueIndexQueryResult,
};
use crate::{
    Connection,
    sqlx_types::{PgPool, SqlxError},
};
use sea_query::{Alias, DynIden, IntoIden, SeaRc};
use std::collections::HashMap;

mod executor;
pub use executor::*;

pub(crate) type EnumVariantMap = HashMap<String, (String, Vec<String>)>;

pub struct SchemaDiscovery {
    pub query: SchemaQueryBuilder,
    pub schema: DynIden,
    exec: Option<Executor>,
}

impl SchemaDiscovery {
    /// Discover schema from a SQLx pool
    pub fn new(pool: PgPool, schema: &str) -> Self {
        SchemaDiscovery {
            query: SchemaQueryBuilder::default(),
            schema: Alias::new(schema).into_iden(),
            exec: Some(pool.into_executor()),
        }
    }

    #[doc(hidden)]
    pub fn new_no_exec(schema: &str) -> Self {
        Self {
            query: SchemaQueryBuilder::default(),
            schema: Alias::new(schema).into_iden(),
            exec: None,
        }
    }

    fn conn(&self) -> Result<&Executor, SqlxError> {
        match &self.exec {
            Some(exec) => Ok(exec),
            None => Err(SqlxError::PoolClosed),
        }
    }

    pub async fn discover(&self) -> Result<Schema, SqlxError> {
        self.discover_with(self.conn()?).await
    }

    #[doc(hidden)]
    pub async fn discover_with<C: Connection>(&self, conn: &C) -> Result<Schema, SqlxError> {
        let enums = self.discover_enums_with(conn).await?;
        let enum_map: EnumVariantMap = enums
            .iter()
            .map(|e| (e.typename.clone(), (e.schema.clone(), e.values.clone())))
            .collect();

        let mut tables = Vec::new();
        for table in self.discover_tables_with(conn).await? {
            tables.push(self.discover_table_with(conn, table, &enum_map).await?);
        }

        Ok(Schema {
            schema: self.schema.to_string(),
            tables,
            enums,
        })
    }

    async fn discover_tables_with<C: Connection>(
        &self,
        conn: &C,
    ) -> Result<Vec<TableInfo>, SqlxError> {
        let rows = conn
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

    async fn discover_table_with<C: Connection>(
        &self,
        conn: &C,
        info: TableInfo,
        enums: &EnumVariantMap,
    ) -> Result<TableDef, SqlxError> {
        let table = SeaRc::new(Alias::new(info.name.as_str()));
        let columns = self
            .discover_columns_with(conn, self.schema.clone(), table.clone(), enums)
            .await?;
        let constraints = self
            .discover_constraints_with(conn, self.schema.clone(), table.clone())
            .await?;
        let (
            check_constraints,
            not_null_constraints,
            primary_key_constraints,
            reference_constraints,
            exclusion_constraints,
        ) = constraints.into_iter().fold(
            (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()),
            |mut acc, constraint| {
                match constraint {
                    Constraint::Check(check) => acc.0.push(check),
                    Constraint::NotNull(not_null) => acc.1.push(not_null),
                    Constraint::Unique(_) => (),
                    Constraint::PrimaryKey(primary_key) => acc.2.push(primary_key),
                    Constraint::References(references) => acc.3.push(references),
                    Constraint::Exclusion(exclusion) => acc.4.push(exclusion),
                }
                acc
            },
        );

        let unique_constraints = self
            .discover_unique_indexes_with(conn, self.schema.clone(), table.clone())
            .await?;

        Ok(TableDef {
            info,
            columns,
            check_constraints,
            not_null_constraints,
            unique_constraints,
            primary_key_constraints,
            reference_constraints,
            exclusion_constraints,
        })
    }

    async fn discover_columns_with<C: Connection>(
        &self,
        conn: &C,
        schema: DynIden,
        table: DynIden,
        enums: &EnumVariantMap,
    ) -> Result<Vec<ColumnInfo>, SqlxError> {
        let rows = conn
            .query_all(self.query.query_columns(schema.clone(), table.clone()))
            .await?;

        Ok(rows
            .into_iter()
            .map(|row| {
                let result: ColumnQueryResult = row.into();
                debug_print!("{:?}", result);
                let column = result.parse(enums);
                debug_print!("{:?}", column);
                column
            })
            .collect())
    }

    async fn discover_constraints_with<C: Connection>(
        &self,
        conn: &C,
        schema: DynIden,
        table: DynIden,
    ) -> Result<Vec<Constraint>, SqlxError> {
        let rows = conn
            .query_all(
                self.query
                    .query_table_constraints(schema.clone(), table.clone()),
            )
            .await?;

        let results = rows
            .into_iter()
            .map(|row| {
                let result: TableConstraintsQueryResult = row.into();
                debug_print!("{:?}", result);
                result
            })
            .collect();

        let results = parse_table_constraint_query_results(results);

        results.iter().for_each(|_index| {
            debug_print!("{:?}", _index);
        });

        Ok(results)
    }

    async fn discover_unique_indexes_with<C: Connection>(
        &self,
        conn: &C,
        schema: DynIden,
        table: DynIden,
    ) -> Result<Vec<Unique>, SqlxError> {
        let rows = conn
            .query_all(
                self.query
                    .query_table_unique_indexes(schema.clone(), table.clone()),
            )
            .await?;

        let results = rows.into_iter().map(|row| {
            let result: UniqueIndexQueryResult = row.into();
            debug_print!("{:?}", result);
            result
        });

        Ok(parse_unique_index_query_results(Box::new(results))
            .inspect(|_index| {
                debug_print!("{:?}", _index);
            })
            .collect())
    }

    pub async fn discover_search_path(&self) -> Result<Vec<String>, SqlxError> {
        self.discover_search_path_with(self.conn()?).await
    }

    #[doc(hidden)]
    pub async fn discover_search_path_with<C: Connection>(
        &self,
        conn: &C,
    ) -> Result<Vec<String>, SqlxError> {
        let rows = conn
            .query_all(self.query.query_search_path())
            .await?;

        let schemas: Vec<String> = rows
            .into_iter()
            .flat_map(|row| {
                let result: SearchPathResult = row.into();
                debug_print!("{:?}", result);
                // filter user-defined schemas (starting with $) and trim quotes
                result
                    .setting
                    .split(',')
                    .map(|s| s.trim().trim_matches('"').to_string()) 
                    .filter(|s| !s.is_empty() && !s.starts_with('$')) 
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(schemas)
    }

    pub async fn discover_enums(&self) -> Result<Vec<EnumDef>, SqlxError> {
        self.discover_enums_with(self.conn()?).await
    }

    #[doc(hidden)]
    pub async fn discover_enums_with<C: Connection>(
        &self,
        conn: &C,
    ) -> Result<Vec<EnumDef>, SqlxError> {
        let current_schema = self.schema.to_string();
        let schemas_to_search = std::iter::once(current_schema.clone())
            .chain(
                self.discover_search_path_with(conn)
                    .await?
                    .into_iter()
                    .filter(|s| s != &current_schema),
            )
            .collect::<Vec<_>>();

        let mut enums_by_typename: HashMap<String, (String, Vec<String>)> = HashMap::new();

        for schema_name in &schemas_to_search {
            let rows = conn
                .query_all(self.query.query_enums(Alias::new(schema_name).into_iden()))
                .await?;

            for row in rows {
                let result: EnumQueryResult = row.into();
                debug_print!("{:?}", result);
                
                enums_by_typename
                    .entry(result.typename)
                    .or_insert((schema_name.clone(), Vec::new()))
                    .1
                    .push(result.enumlabel);
            }
        }

        Ok(enums_by_typename
            .into_iter()
            .map(|(typename, (schema, values))| EnumDef {
                typename,
                schema,
                values,
            })
            .collect())
    }
}
