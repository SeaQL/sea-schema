use sea_query::{Alias, Condition, Expr, IntoTableRef, Query, SelectStatement, SimpleExpr};

use super::Postgres;
use crate::probe::SchemaProbe;

impl SchemaProbe for Postgres {
    fn get_current_schema() -> SimpleExpr {
        Expr::cust("CURRENT_SCHEMA()")
    }

    fn query_tables() -> SelectStatement {
        let (expr, tbl_ref, condition) = (
            Expr::col(Alias::new("table_name")),
            (Alias::new("information_schema"), Alias::new("tables")).into_table_ref(),
            Condition::all()
                .add(
                    Expr::expr(Self::get_current_schema())
                        .equals(Alias::new("tables"), Alias::new("table_schema")),
                )
                .add(Expr::col(Alias::new("table_type")).eq("BASE TABLE")),
        );
        Query::select()
            .expr_as(expr, Alias::new("table_name"))
            .from(tbl_ref)
            .cond_where(condition)
            .take()
    }
}
