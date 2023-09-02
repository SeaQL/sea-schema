use sea_query::{Condition, Expr, Iden, Query, SelectStatement, SimpleExpr};

pub trait SchemaProbe {
    fn get_current_schema() -> SimpleExpr;

    fn query_tables() -> SelectStatement;

    fn has_table<T>(table: T) -> SelectStatement
    where
        T: AsRef<str>,
    {
        let mut subquery = Self::query_tables();
        subquery.cond_where(Expr::col(DatabaseSchema::TableName).eq(table.as_ref()));
        Query::select()
            .expr_as(Expr::cust("COUNT(*) > 0"), Has::Table)
            .from_subquery(subquery, Subquery)
            .take()
    }

    fn has_column<T, C>(table: T, column: C) -> SelectStatement
    where
        T: AsRef<str>,
        C: AsRef<str>,
    {
        Query::select()
            .expr_as(Expr::cust("COUNT(*) > 0"), Has::Column)
            .from((DatabaseSchema::Info, DatabaseSchema::Columns))
            .cond_where(
                Condition::all()
                    .add(
                        Expr::expr(Self::get_current_schema())
                            .equals((DatabaseSchema::Columns, DatabaseSchema::TableSchema)),
                    )
                    .add(Expr::col(DatabaseSchema::TableName).eq(table.as_ref()))
                    .add(Expr::col(DatabaseSchema::ColumnName).eq(column.as_ref())),
            )
            .take()
    }

    fn has_index<T, C>(table: T, index: C) -> SelectStatement
    where
        T: AsRef<str>,
        C: AsRef<str>;
}

#[derive(Debug, Iden)]
pub enum Has {
    #[iden = "has_table"]
    Table,
    #[iden = "has_column"]
    Column,
    #[iden = "has_index"]
    Index,
}

#[derive(Debug, Iden)]
pub enum DatabaseSchema {
    #[iden = "information_schema"]
    Info,
    Columns,
    TableName,
    ColumnName,
    TableSchema,
}

#[derive(Debug, Iden)]
struct Subquery;
