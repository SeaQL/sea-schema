use sea_query::{Condition, Expr, Iden, IntoTableRef, Query, SelectStatement, SimpleExpr};

use super::query::SqliteMaster;
use super::Sqlite;
use crate::probe::SchemaProbe;

impl SchemaProbe for Sqlite {
    fn get_current_schema() -> SimpleExpr {
        unimplemented!()
    }

    fn query_tables() -> SelectStatement {
        Query::select()
            .expr_as(Expr::col(Schema::Name), Schema::TableName)
            .from(SqliteMaster.into_table_ref())
            .cond_where(
                Condition::all()
                    .add(Expr::col(Schema::Type).eq("table"))
                    .add(Expr::col(Schema::Name).ne("sqlite_sequence")),
            )
            .take()
    }

    fn has_column<T, C>(table: T, column: C) -> SelectStatement
    where
        T: AsRef<str>,
        C: AsRef<str>,
    {
        Query::select()
            .expr(Expr::cust_with_values(
                "COUNT(*) > 0 AS \"has_column\" FROM pragma_table_info(?)",
                [table.as_ref()],
            ))
            .and_where(Expr::col(Schema::Name).eq(column.as_ref()))
            .take()
    }
}

#[derive(Debug, Iden)]
enum Schema {
    Name,
    Type,
    TableName,
}
