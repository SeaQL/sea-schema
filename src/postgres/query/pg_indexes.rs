use super::SchemaQueryBuilder;
use crate::sqlx_types::SqlxRow;
use sea_query::{
    Condition, DynIden, Expr, ExprTrait, Iden, JoinType, Order, Query, SelectStatement,
};

#[derive(Debug, Iden)]
pub enum PgIndexes {
    Table,
    #[iden = "tablename"]
    TableName,
    #[iden = "schemaname"]
    SchemaName,
    #[iden = "indexname"]
    IndexName,
}

#[derive(Debug, Iden)]
pub enum PgIndex {
    Table,
    #[iden = "indexrelid"]
    IndexRelId,
    #[iden = "indrelid"]
    IndRelId,
    #[iden = "indisunique"]
    IndIsUnique,
    #[iden = "indisprimary"]
    IndIsPrimary,
    #[iden = "indpred"]
    IndPred,
}

#[derive(Debug, Iden)]
pub enum PgClass {
    Table,
    Oid,
    #[iden = "relnamespace"]
    RelNamespace,
    #[iden = "relname"]
    RelName,
}

#[derive(Debug, Iden)]
pub enum PgNamespace {
    Table,
    Oid,
    #[iden = "nspname"]
    NspName,
}

#[derive(Debug, Iden)]
pub enum PgAttribute {
    Table,
    Oid,
    #[iden = "attrelid"]
    AttRelId,
    #[iden = "attname"]
    AttName,
}

#[derive(Debug, Default)]
pub struct UniqueIndexQueryResult {
    pub index_name: String,
    pub table_schema: String,
    pub table_name: String,
    pub column_name: String,
    pub is_partial: bool,
}

impl SchemaQueryBuilder {
    pub fn query_table_unique_indexes(&self, schema: DynIden, table: DynIden) -> SelectStatement {
        let idx = "idx";
        let insp = "insp";
        let tbl = "tbl";
        let tnsp = "tnsp";
        let col = "col";
        let partially = "partially";

        Query::select()
            .column((idx, PgClass::RelName))
            .column((insp, PgNamespace::NspName))
            .column((tbl, PgClass::RelName))
            .column((col, PgAttribute::AttName))
            .expr_as(Expr::col(PgIndex::IndPred).is_not_null(), partially)
            .from(PgIndex::Table)
            .join_as(
                JoinType::Join,
                PgClass::Table,
                idx,
                Expr::col((idx, PgClass::Oid)).equals((PgIndex::Table, PgIndex::IndexRelId)),
            )
            .join_as(
                JoinType::Join,
                PgNamespace::Table,
                insp,
                Expr::col((insp, PgNamespace::Oid)).equals((idx, PgClass::RelNamespace)),
            )
            .join_as(
                JoinType::Join,
                PgClass::Table,
                tbl,
                Expr::col((tbl, PgClass::Oid)).equals((PgIndex::Table, PgIndex::IndRelId)),
            )
            .join_as(
                JoinType::Join,
                PgNamespace::Table,
                tnsp,
                Expr::col((tnsp, PgNamespace::Oid)).equals((tbl, PgClass::RelNamespace)),
            )
            .join_as(
                JoinType::Join,
                PgAttribute::Table,
                col,
                Expr::col((col, PgAttribute::AttRelId)).equals((idx, PgAttribute::Oid)),
            )
            .cond_where(
                Condition::all()
                    .add(Expr::col((PgIndex::Table, PgIndex::IndIsUnique)).eq(true))
                    .add(Expr::col((PgIndex::Table, PgIndex::IndIsPrimary)).eq(false))
                    .add(Expr::col((tbl, PgClass::RelName)).eq(table.to_string()))
                    .add(Expr::col((tnsp, PgNamespace::NspName)).eq(schema.to_string())),
            )
            .order_by((PgIndex::Table, PgIndex::IndexRelId), Order::Asc)
            .take()
    }
}

#[cfg(feature = "sqlx-postgres")]
impl From<SqlxRow> for UniqueIndexQueryResult {
    fn from(row: SqlxRow) -> Self {
        use crate::sqlx_types::Row;
        let row = row.postgres();
        Self {
            index_name: row.get(0),
            table_schema: row.get(1),
            table_name: row.get(2),
            column_name: row.get(3),
            is_partial: row.get(4),
        }
    }
}

#[cfg(not(feature = "sqlx-postgres"))]
impl From<SqlxRow> for UniqueIndexQueryResult {
    fn from(_: SqlxRow) -> Self {
        Self::default()
    }
}
