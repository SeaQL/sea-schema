use sea_query::{Alias, Condition, Expr, Query, SelectStatement, SimpleExpr};

use super::query::{InformationSchema as Schema, TablesFields};
use super::MySql;
use crate::probe::SchemaProbe;

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
            .expr_as(Expr::cust("COUNT(*) > 0"), Alias::new("has_index"))
            .from((Alias::new("information_schema"), Alias::new("statistics")))
            .cond_where(
                Condition::all()
                    .add(Expr::col(Alias::new("table_schema")).eq(Self::get_current_schema()))
                    .add(Expr::col(Alias::new("table_name")).eq(table.as_ref()))
                    .add(Expr::col(Alias::new("index_name")).eq(index.as_ref())),
            )
            .take()
    }
}
