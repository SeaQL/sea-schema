use sea_query::{Alias, Condition, Expr, Query, SelectStatement, SimpleExpr};

pub trait SchemaProbe {
    fn get_current_schema() -> SimpleExpr;

    fn query_tables() -> SelectStatement;

    fn has_table<T>(table: T) -> SelectStatement
    where
        T: AsRef<str>,
    {
        let mut subquery = Self::query_tables();
        subquery.cond_where(Expr::col(Alias::new("table_name")).eq(table.as_ref()));
        Query::select()
            .expr_as(Expr::cust("COUNT(*) > 0"), Alias::new("has_table"))
            .from_subquery(subquery, Alias::new("subquery"))
            .take()
    }

    fn has_column<T, C>(table: T, column: C) -> SelectStatement
    where
        T: AsRef<str>,
        C: AsRef<str>,
    {
        Query::select()
            .expr_as(Expr::cust("COUNT(*) > 0"), Alias::new("has_column"))
            .from((Alias::new("information_schema"), Alias::new("columns")))
            .cond_where(
                Condition::all()
                    .add(
                        Expr::expr(Self::get_current_schema())
                            .equals(Alias::new("columns"), Alias::new("table_schema")),
                    )
                    .add(Expr::col(Alias::new("table_name")).eq(table.as_ref()))
                    .add(Expr::col(Alias::new("column_name")).eq(column.as_ref())),
            )
            .take()
    }
}
