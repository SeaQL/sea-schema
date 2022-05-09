use sea_query::{Alias, Condition, Expr, IntoTableRef, Query, SelectStatement, SimpleExpr};

use super::Sqlite;
use crate::probe::SchemaProbe;

impl SchemaProbe for Sqlite {
    fn get_current_schema() -> SimpleExpr {
        unimplemented!()
    }

    fn query_tables() -> SelectStatement {
        let mut stmt = Query::select();
        let (expr, tbl_ref, condition) = (
            Expr::col(Alias::new("name")),
            Alias::new("sqlite_master").into_table_ref(),
            Condition::all()
                .add(Expr::col(Alias::new("type")).eq("table"))
                .add(Expr::col(Alias::new("name")).ne("sqlite_sequence")),
        );
        stmt.expr_as(expr, Alias::new("table_name"))
            .from(tbl_ref)
            .cond_where(condition);
        stmt
    }
}
