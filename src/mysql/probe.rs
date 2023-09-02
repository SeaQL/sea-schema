use sea_query::{Condition, Expr, Iden, Query, SelectStatement, SimpleExpr};

use super::query::{InformationSchema as Schema, TablesFields};
use super::MySql;
use crate::mysql::query::InformationSchema;
use crate::probe::{DatabaseSchema, Has, SchemaProbe};

impl SchemaProbe for MySql {
    fn get_current_schema() -> SimpleExpr {
        Expr::cust("DATABASE()")
    }

    fn query_tables() -> SelectStatement {
        Query::select()
            .expr_as(Expr::col(TablesFields::TableName), TablesFields::TableName)
            .from((Schema::Schema, Schema::Tables))
            .cond_where(
                Condition::all().add(
                    Expr::expr(Self::get_current_schema())
                        .equals((Schema::Tables, TablesFields::TableSchema)),
                ),
            )
            .take()
    }

    fn has_index<T, C>(table: T, index: C) -> SelectStatement
    where
        T: AsRef<str>,
        C: AsRef<str>,
    {
        Query::select()
            .expr_as(Expr::cust("COUNT(*) > 0"), Has::Index)
            .from((DatabaseSchema::Info, InformationSchema::Statistics))
            .cond_where(
                Condition::all()
                    .add(Expr::col(DatabaseSchema::TableSchema).eq(Self::get_current_schema()))
                    .add(Expr::col(DatabaseSchema::TableName).eq(table.as_ref()))
                    .add(Expr::col(InternalDatabaseSchema::IndexName).eq(index.as_ref())),
            )
            .take()
    }
}

#[derive(Debug, Iden)]
pub enum InternalDatabaseSchema {
    IndexName,
}
