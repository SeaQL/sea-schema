use super::{InformationSchema, SchemaQueryBuilder};
use crate::sqlx_types::postgres::PgRow;
use sea_query::{Expr, Iden, Query, SeaRc, SelectStatement};

#[derive(Debug, sea_query::Iden)]
pub enum PgType {
    #[iden = "pg_type"]
    Table,
    #[iden = "typname"]
    TypeName,
    #[iden = "oid"]
    Oid,
}

#[derive(Debug, sea_query::Iden)]
pub enum PgEnum {
    #[iden = "pg_enum"]
    Table,
    #[iden = "enumlabel"]
    EnumLabel,
    #[iden = "enumtypid"]
    EnumTypeId,
}

#[derive(Debug, Default)]
pub struct EnumQueryResult {
    pub typename: String,
    pub enumlabel: String,
}

impl SchemaQueryBuilder {
    pub fn query_enums(&self) -> SelectStatement {
        Query::select()
            .column((PgType::Table, PgType::TypeName))
            .column((PgEnum::Table, PgEnum::EnumLabel))
            .from(PgType::Table)
            .inner_join(
                PgEnum::Table,
                Expr::tbl(PgEnum::Table, PgEnum::EnumTypeId).equals(PgType::Table, PgType::Oid),
            )
            .take()
    }
}

#[cfg(feature = "sqlx-postgres")]
impl From<&PgRow> for EnumQueryResult {
    fn from(row: &PgRow) -> Self {
        use crate::sqlx_types::Row;
        Self {
            typename: row.get(0),
            enumlabel: row.get(1),
        }
    }
}

#[cfg(not(feature = "sqlx-postgres"))]
impl From<&PgRow> for EnumQueryResult {
    fn from(row: &PgRow) -> Self {
        Self::default()
    }
}