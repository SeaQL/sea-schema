use super::SchemaQueryBuilder;
use crate::{postgres::query::PgNamespace, sqlx_types::SqlxRow};
use sea_query::{DynIden, Expr, ExprTrait, Order, Query, SelectStatement};

#[derive(Debug, sea_query::Iden)]
pub enum PgCatalog {
    #[iden = "pg_catalog"]
    Schema,
}

#[derive(Debug, sea_query::Iden)]
pub enum PgType {
    #[iden = "pg_type"]
    Table,
    #[iden = "typname"]
    TypeName,
    #[iden = "oid"]
    Oid,
    #[iden = "typnamespace"]
    TypeNamespace,
}

#[derive(Debug, sea_query::Iden)]
pub enum PgEnum {
    #[iden = "pg_enum"]
    Table,
    #[iden = "enumlabel"]
    EnumLabel,
    #[iden = "enumtypid"]
    EnumTypeId,
    #[iden = "enumsortorder"]
    EnumSortOrder,
}

#[derive(Debug, Default)]
pub struct EnumQueryResult {
    pub typename: String,
    pub enumlabel: String,
}

impl SchemaQueryBuilder {
    pub fn query_enums(&self, schema: DynIden) -> SelectStatement {
        Query::select()
            .column((PgType::Table, PgType::TypeName))
            .column((PgEnum::Table, PgEnum::EnumLabel))
            .from((PgCatalog::Schema, PgType::Table))
            .inner_join(
                (PgCatalog::Schema, PgEnum::Table),
                Expr::col((PgEnum::Table, PgEnum::EnumTypeId)).equals((PgType::Table, PgType::Oid)),
            )
            .inner_join(
                (PgCatalog::Schema, PgNamespace::Table),
                Expr::col((PgNamespace::Table, PgNamespace::Oid))
                    .equals((PgType::Table, PgType::TypeNamespace)),
            )
            .and_where(Expr::col((PgNamespace::Table, PgNamespace::NspName)).eq(schema.to_string()))
            .order_by((PgType::Table, PgType::TypeName), Order::Asc)
            .order_by((PgEnum::Table, PgEnum::EnumSortOrder), Order::Asc)
            .order_by((PgEnum::Table, PgEnum::EnumLabel), Order::Asc)
            .take()
    }
}

#[cfg(feature = "sqlx-postgres")]
impl From<SqlxRow> for EnumQueryResult {
    fn from(row: SqlxRow) -> Self {
        use crate::sqlx_types::Row;
        let row = row.postgres();
        Self {
            typename: row.get(0),
            enumlabel: row.get(1),
        }
    }
}

#[cfg(not(feature = "sqlx-postgres"))]
impl From<SqlxRow> for EnumQueryResult {
    fn from(_: SqlxRow) -> Self {
        Self::default()
    }
}
