use sea_query::{Condition, Expr, Query, SelectStatement, SimpleExpr};

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
                        .equals(Schema::Tables, TablesFields::TableSchema),
                ),
            )
            .take()
    }
}
