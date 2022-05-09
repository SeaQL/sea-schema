use sea_query::{SelectStatement, SimpleExpr};

pub trait SchemaProbe {
    fn get_current_schema() -> SimpleExpr;

    fn query_tables() -> SelectStatement;
}
