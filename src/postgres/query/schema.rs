use crate::sqlx_types::SqlxRow;
use sea_query::{Condition, Expr, ExprTrait, Iden, JoinType, Query, SelectStatement};

#[derive(Debug, Default)]
pub struct SchemaQueryBuilder {}

impl SchemaQueryBuilder {
    pub fn query_search_path(&self) -> SelectStatement {
        select_search_path()
    }
}

#[derive(Debug, Iden)]
/// Ref: https://www.postgresql.org/docs/13/information-schema.html
pub enum InformationSchema {
    #[iden = "information_schema"]
    Schema,
    Columns,
    CheckConstraints,
    KeyColumnUsage,
    ReferentialConstraints,
    Tables,
    TableConstraints,
    ConstraintColumnUsage,
}

pub(crate) fn select_base_table_and_view() -> SelectStatement {
    #[derive(Debug, Iden)]
    enum PgClass {
        Table,
        Relname,
        Relkind,
        Oid,
    }

    #[derive(Debug, Iden)]
    enum PgInherits {
        Table,
        Inhrelid,
    }

    Query::select()
        .column((PgClass::Table, PgClass::Relname))
        .from(PgInherits::Table)
        .join(
            JoinType::Join,
            PgClass::Table,
            Condition::all()
                .add(
                    Expr::col((PgInherits::Table, PgInherits::Inhrelid))
                        .equals((PgClass::Table, PgClass::Oid)),
                )
                .add(
                    // List of possible value of the `relkind` column.
                    // ========
                    // r = ordinary table
                    // i = index
                    // S = sequence
                    // t = TOAST table
                    // v = view
                    // m = materialized view
                    // c = composite type
                    // f = foreign table
                    // p = partitioned table
                    // I = partitioned index
                    // Extracted from https://www.postgresql.org/docs/current/catalog-pg-class.html
                    //
                    // We want to select tables and views only.
                    Expr::col((PgClass::Table, PgClass::Relkind))
                        .is_in(["r", "t", "v", "m", "f", "p"]),
                ),
        )
        .to_owned()
}

pub(crate) fn select_search_path() -> SelectStatement {
    #[derive(Iden)]
    pub enum PgSettings {
        #[iden = "pg_settings"]
        Table,
        #[iden = "name"]
        Name,
        #[iden = "setting"]
        Setting,
    }

    Query::select()
        .column(PgSettings::Setting)
        .from(PgSettings::Table)
        .and_where(Expr::col(PgSettings::Name).eq("search_path"))
        .to_owned()
}

#[derive(Debug, Default)]
pub struct SearchPathResult {
    pub setting: String,
}

#[cfg(feature = "sqlx-postgres")]
impl From<SqlxRow> for SearchPathResult {
    fn from(row: SqlxRow) -> Self {
        use crate::sqlx_types::Row;
        let row = row.postgres();
        Self {
            setting: row.get(0),
        }
    }
}

#[cfg(not(feature = "sqlx-postgres"))]
impl From<SqlxRow> for SearchPathResult {
    fn from(_: SqlxRow) -> Self {
        Self::default()
    }
}
