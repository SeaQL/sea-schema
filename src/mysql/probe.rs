use sea_query::{Alias, Condition, Expr, IntoTableRef, Query, SelectStatement, SimpleExpr};

use super::MySql;
use crate::probe::SchemaProbe;

impl SchemaProbe for MySql {
    fn get_current_schema() -> SimpleExpr {
        Expr::cust("DATABASE()")
    }

    fn query_tables() -> SelectStatement {
        let (expr, tbl_ref, condition) = (
            Expr::col(Alias::new("table_name")),
            (Alias::new("information_schema"), Alias::new("tables")).into_table_ref(),
            Condition::all().add(
                Expr::expr(Self::get_current_schema())
                    .equals(Alias::new("tables"), Alias::new("table_schema")),
            ),
        );
        Query::select()
            .expr_as(expr, Alias::new("table_name"))
            .from(tbl_ref)
            .cond_where(condition)
            .take()
    }
}
