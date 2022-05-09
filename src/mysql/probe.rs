use sea_query::{Condition, Expr, IntoTableRef, Query, SelectStatement, SimpleExpr};

use super::query::{InformationSchema as Schema, TablesFields};
use super::MySql;
use crate::probe::SchemaProbe;

impl SchemaProbe for MySql {
    fn get_current_schema() -> SimpleExpr {
        Expr::cust("DATABASE()")
    }

    fn query_tables() -> SelectStatement {
        let (expr, tbl_ref, condition) = (
            Expr::col(TablesFields::TableName),
            (Schema::Schema, Schema::Tables).into_table_ref(),
            Condition::all().add(
                Expr::expr(Self::get_current_schema())
                    .equals(Schema::Tables, TablesFields::TableSchema),
            ),
        );
        Query::select()
            .expr_as(expr, TablesFields::TableName)
            .from(tbl_ref)
            .cond_where(condition)
            .take()
    }
}
