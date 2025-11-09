//! To query & parse MySQL's INFORMATION_SCHEMA and construct a [`Schema`]

use crate::debug_print;
use crate::postgres::def::*;
use crate::postgres::parser::{
    parse_table_constraint_query_results, parse_unique_index_query_results,
};
use crate::postgres::query::{
    ColumnQueryResult, EnumQueryResult, SchemaQueryBuilder, TableConstraintsQueryResult,
    TableQueryResult, UniqueIndexQueryResult,
};
use crate::{
    Connection,
    sqlx_types::{PgPool, SqlxError},
};
use sea_query::{Alias, DynIden, IntoIden, SeaRc};
use std::collections::HashMap;

mod executor;
pub use executor::*;

pub(crate) type EnumVariantMap = HashMap<String, Vec<String>>;

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
            .map(|enum_def| (enum_def.typename.clone(), enum_def.values.clone()))
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

        let results = rows.into_iter().map(|row| {
            let result: TableConstraintsQueryResult = row.into();
            debug_print!("{:?}", result);
            result
        });

        Ok(parse_table_constraint_query_results(Box::new(results))
            .inspect(|_index| {
                debug_print!("{:?}", _index);
            })
            .collect())
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

    pub async fn discover_enums(&self) -> Result<Vec<EnumDef>, SqlxError> {
        self.discover_enums_with(self.conn()?).await
    }

    #[doc(hidden)]
    pub async fn discover_enums_with<C: Connection>(
        &self,
        conn: &C,
    ) -> Result<Vec<EnumDef>, SqlxError> {
        let rows = conn.query_all(self.query.query_enums()).await?;

        let enum_rows = rows.into_iter().map(|row| {
            let result: EnumQueryResult = row.into();
            debug_print!("{:?}", result);
            result
        });

        let map = enum_rows.fold(
            HashMap::new(),
            |mut map: HashMap<String, Vec<String>>,
             EnumQueryResult {
                 typename,
                 enumlabel,
             }| {
                if let Some(entry_exists) = map.get_mut(&typename) {
                    entry_exists.push(enumlabel);
                } else {
                    map.insert(typename, vec![enumlabel]);
                }
                map
            },
        );

        Ok(map
            .into_iter()
            .map(|(typename, values)| EnumDef { values, typename })
            .collect())
    }
}
