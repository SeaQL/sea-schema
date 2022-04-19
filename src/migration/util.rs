use super::{ConnectionTrait, DbBackend};
use sea_query::{Alias, Condition, Expr, IntoTableRef, Query, SelectStatement, SimpleExpr};

pub(crate) fn query_tables(conn: &dyn ConnectionTrait) -> SelectStatement {
    let mut stmt = Query::select();
    let (expr, tbl_ref, condition) = match conn.get_database_backend() {
        DbBackend::MySql => (
            Expr::col(Alias::new("table_name")),
            (Alias::new("information_schema"), Alias::new("tables")).into_table_ref(),
            Condition::all().add(
                Expr::expr(get_current_schema(conn))
                    .equals(Alias::new("tables"), Alias::new("table_schema")),
            ),
        ),
        DbBackend::Postgres => (
            Expr::col(Alias::new("table_name")),
            (Alias::new("information_schema"), Alias::new("tables")).into_table_ref(),
            Condition::all()
                .add(
                    Expr::expr(get_current_schema(conn))
                        .equals(Alias::new("tables"), Alias::new("table_schema")),
                )
                .add(Expr::col(Alias::new("table_type")).eq("BASE TABLE")),
        ),
        DbBackend::Sqlite => (
            Expr::col(Alias::new("name")),
            Alias::new("sqlite_master").into_table_ref(),
            Condition::all()
                .add(Expr::col(Alias::new("type")).eq("table"))
                .add(Expr::col(Alias::new("name")).ne("sqlite_sequence")),
        ),
    };
    stmt.expr_as(expr, Alias::new("table_name"))
        .from(tbl_ref)
        .cond_where(condition);
    stmt
}

pub(crate) fn get_current_schema(conn: &dyn ConnectionTrait) -> SimpleExpr {
    match conn.get_database_backend() {
        DbBackend::MySql => Expr::cust("DATABASE()"),
        DbBackend::Postgres => Expr::cust("CURRENT_SCHEMA()"),
        DbBackend::Sqlite => unimplemented!(),
    }
}
