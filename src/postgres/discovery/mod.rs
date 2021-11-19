//! To query & parse MySQL's INFORMATION_SCHEMA and construct a [`Schema`]

use crate::debug_print;
use crate::postgres::def::*;
use crate::postgres::parser::parse_table_constraint_query_results;
use crate::postgres::query::{
    ColumnQueryResult, SchemaQueryBuilder, TableConstraintsQueryResult, TableQueryResult,
};
use futures::future;
use sea_query::{Alias, ColumnRef, Expr, Iden, IntoIden, JoinType, SeaRc, SelectStatement};
use std::collections::HashMap;

mod executor;
pub use executor::*;

pub struct SchemaDiscovery {
    pub query: SchemaQueryBuilder,
    pub executor: Executor,
    pub schema: SeaRc<dyn Iden>,
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

        tables
    }

    async fn discover_table_static(params: (&Self, TableInfo)) -> TableDef {
        let this = params.0;
        let info = params.1;
        Self::discover_table(this, info).await
    }

    pub async fn discover_table(&self, info: TableInfo) -> TableDef {
        let table = SeaRc::new(Alias::new(info.name.as_str()));
        let columns = self
            .discover_columns(self.schema.clone(), table.clone())
            .await;
        let constraints = self
            .discover_constraints(self.schema.clone(), table.clone())
            .await;
        let (
            check_constraints,
            not_null_constraints,
            unique_constraints,
            primary_key_constraints,
            reference_constraints,
            exclusion_constraints,
        ) = constraints.into_iter().fold(
            (
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
            |mut acc, constraint| {
                match constraint {
                    Constraint::Check(check) => acc.0.push(check),
                    Constraint::NotNull(not_null) => acc.1.push(not_null),
                    Constraint::Unique(unique) => acc.2.push(unique),
                    Constraint::PrimaryKey(primary_key) => acc.3.push(primary_key),
                    Constraint::References(references) => acc.4.push(references),
                    Constraint::Exclusion(exclusion) => acc.5.push(exclusion),
                }
                acc
            },
        );

        TableDef {
            info,
            columns,
            check_constraints,
            not_null_constraints,
            unique_constraints,
            primary_key_constraints,
            reference_constraints,
            exclusion_constraints,
        }
    }

    pub async fn discover_columns(
        &self,
        schema: SeaRc<dyn Iden>,
        table: SeaRc<dyn Iden>,
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

        columns
    }

    pub async fn discover_constraints(
        &self,
        schema: SeaRc<dyn Iden>,
        table: SeaRc<dyn Iden>,
    ) -> Vec<Constraint> {
        let rows = self
            .executor
            .fetch_all(
                self.query
                    .query_table_constriants(schema.clone(), table.clone()),
            )
            .await;

        let results: Vec<TableConstraintsQueryResult> = rows
            .iter()
            .map(|row| {
                let result = row.into();
                debug_print!("{:?}", result);
                result
            })
            .collect();

        let constraints = parse_table_constraint_query_results(Box::new(results.into_iter()))
            .map(|index| {
                debug_print!("{:?}", index);
                index
            })
            .collect::<Vec<_>>();

        constraints
    }

    #[cfg(feature = "sqlx-postgres")]
    pub async fn discover_enums(&self) -> Vec<Type> {
        let mut query = SelectStatement::new();

        // SELECT pg_type.typname, pg_enum.enumlabel FROM pg_type JOIN pg_enum ON pg_enum.enumtypid = pg_type.oid;
        query
            .expr(Expr::col(ColumnRef::TableColumn(
                SeaRc::new(Alias::new("pg_type")),
                SeaRc::new(Alias::new("typname")),
            )))
            .expr(Expr::col(ColumnRef::TableColumn(
                SeaRc::new(Alias::new("pg_enum")),
                SeaRc::new(Alias::new("enumlabel")),
            )))
            .from(Alias::new("pg_type"))
            .join(
                JoinType::Join,
                Alias::new("pg_enum"),
                Expr::tbl(Alias::new("pg_enum"), Alias::new("enumtypid"))
                    .equals(Alias::new("pg_type"), Alias::new("oid")),
            );
        let enum_rows = self.executor.get_enums(query).await;

        let mut enums: Vec<Type> = Vec::default();

        let mut temp_enumdef: HashMap<Typname, EnumValues> = HashMap::new();

        enum_rows.iter().for_each(|enum_row| {
            if let Some(entry_exists) = temp_enumdef.get_mut(&enum_row.typname) {
                entry_exists.push(enum_row.enumlabel.to_owned());
            } else {
                temp_enumdef.insert(
                    enum_row.typname.to_owned(),
                    vec![enum_row.enumlabel.to_owned()],
                );
            }
        });

        temp_enumdef.into_iter().for_each(|key_value_pair| {
            let mut pg_enum = Type::Enum(EnumDef::default());
            pg_enum.get_enum_def_mut().typename = key_value_pair.0.clone();

            let attr_len = key_value_pair.0.len() as u16;
            pg_enum.get_enum_def_mut().attr.length = Some(attr_len);

            pg_enum.get_enum_def_mut().values = key_value_pair.1;

            enums.push(pg_enum);
        });

        enums
    }
}

type Typname = String;
type EnumValues = Vec<String>;
