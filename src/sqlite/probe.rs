use sea_query::{Alias, Condition, Expr, IntoTableRef, Query, SelectStatement, SimpleExpr};

use super::Sqlite;
use crate::probe::SchemaProbe;

impl SchemaProbe for Sqlite {
    fn get_current_schema() -> SimpleExpr {
        unimplemented!()
    }

    fn query_tables() -> SelectStatement {
        let (expr, tbl_ref, condition) = (
            Expr::col(Alias::new("name")),
            Alias::new("sqlite_master").into_table_ref(),
            Condition::all()
                .add(Expr::col(Alias::new("type")).eq("table"))
                .add(Expr::col(Alias::new("name")).ne("sqlite_sequence")),
        );
        Query::select()
            .expr_as(expr, Alias::new("table_name"))
            .from(tbl_ref)
            .cond_where(condition)
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
            .and_where(Expr::col(Alias::new("name")).eq(column.as_ref()))
            .take()
    }
}
