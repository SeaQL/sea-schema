use super::{PgAttribute, PgClass, PgNamespace, SchemaQueryBuilder};
use crate::sqlx_types::SqlxRow;
use sea_query::{
    Condition, DynIden, Expr, ExprTrait, Iden, JoinType, Order, Query, SelectStatement,
};

#[derive(Debug, Iden)]
pub enum PgConstraint {
    Table,
    #[iden = "conname"]
    ConName,
    #[iden = "conrelid"]
    ConRelId,
    #[iden = "confrelid"]
    ConfRelId,
    #[iden = "contype"]
    ConType,
    #[iden = "confupdtype"]
    ConfUpdType,
    #[iden = "confdeltype"]
    ConfDelType,
}

#[derive(Debug, Default)]
pub struct ForeignKeyQueryResult {
    pub constraint_name: String,
    pub column_name: String,
    pub foreign_table_name: String,
    pub foreign_column_name: String,
    pub on_update: Option<String>,
    pub on_delete: Option<String>,
}

impl SchemaQueryBuilder {
    pub fn query_table_foreign_keys(&self, schema: DynIden, table: DynIden) -> SelectStatement {
        let tbl = "tbl";
        let nsp = "nsp";
        let reftbl = "reftbl";
        let col = "col";
        let refcol = "refcol";
        let k = "k";

        Query::select()
            .column((PgConstraint::Table, PgConstraint::ConName))
            .column((col, PgAttribute::AttName))
            .column((reftbl, PgClass::RelName))
            .column((refcol, PgAttribute::AttName))
            // `confupdtype` / `confdeltype` are the internal `"char"` type; cast to
            // text so the single-character action code decodes as a string, then map
            // it to a `ForeignKeyAction` while parsing.
            .expr_as(
                Expr::col((PgConstraint::Table, PgConstraint::ConfUpdType)).cast_as(Text),
                "on_update",
            )
            .expr_as(
                Expr::col((PgConstraint::Table, PgConstraint::ConfDelType)).cast_as(Text),
                "on_delete",
            )
            .from(PgConstraint::Table)
            .join_as(
                JoinType::Join,
                PgClass::Table,
                tbl,
                Expr::col((tbl, PgClass::Oid))
                    .equals((PgConstraint::Table, PgConstraint::ConRelId)),
            )
            .join_as(
                JoinType::Join,
                PgNamespace::Table,
                nsp,
                Expr::col((nsp, PgNamespace::Oid)).equals((tbl, PgClass::RelNamespace)),
            )
            .join_as(
                JoinType::Join,
                PgClass::Table,
                reftbl,
                Expr::col((reftbl, PgClass::Oid))
                    .equals((PgConstraint::Table, PgConstraint::ConfRelId)),
            )
            // `conkey` / `confkey` are parallel arrays of attribute numbers; walk a
            // shared subscript so the local and referenced columns stay paired in
            // order (this is what `unnest(..) WITH ORDINALITY` would express).
            .join_lateral(
                JoinType::Join,
                Query::select()
                    .expr_as(
                        Expr::cust("generate_subscripts(pg_constraint.conkey, 1)"),
                        "ord",
                    )
                    .take(),
                k,
                Expr::cust("true"),
            )
            .join_as(
                JoinType::Join,
                PgAttribute::Table,
                col,
                Condition::all()
                    .add(
                        Expr::col((col, PgAttribute::AttRelId))
                            .equals((PgConstraint::Table, PgConstraint::ConRelId)),
                    )
                    .add(
                        Expr::col((col, PgAttribute::AttNum))
                            .eq(Expr::cust("pg_constraint.conkey[k.ord]")),
                    ),
            )
            .join_as(
                JoinType::Join,
                PgAttribute::Table,
                refcol,
                Condition::all()
                    .add(
                        Expr::col((refcol, PgAttribute::AttRelId))
                            .equals((PgConstraint::Table, PgConstraint::ConfRelId)),
                    )
                    .add(
                        Expr::col((refcol, PgAttribute::AttNum))
                            .eq(Expr::cust("pg_constraint.confkey[k.ord]")),
                    ),
            )
            .cond_where(
                Condition::all()
                    .add(Expr::col((PgConstraint::Table, PgConstraint::ConType)).eq("f"))
                    .add(Expr::col((nsp, PgNamespace::NspName)).eq(schema.to_string()))
                    .add(Expr::col((tbl, PgClass::RelName)).eq(table.to_string())),
            )
            .order_by((PgConstraint::Table, PgConstraint::ConName), Order::Asc)
            .order_by((k, "ord"), Order::Asc)
            .take()
    }
}

#[cfg(feature = "sqlx-postgres")]
impl From<SqlxRow> for ForeignKeyQueryResult {
    fn from(row: SqlxRow) -> Self {
        use crate::sqlx_types::Row;
        let row = row.postgres();
        Self {
            constraint_name: row.get(0),
            column_name: row.get(1),
            foreign_table_name: row.get(2),
            foreign_column_name: row.get(3),
            on_update: row.get(4),
            on_delete: row.get(5),
        }
    }
}

#[cfg(not(feature = "sqlx-postgres"))]
impl From<SqlxRow> for ForeignKeyQueryResult {
    fn from(_: SqlxRow) -> Self {
        Self::default()
    }
}

#[derive(Iden)]
struct Text;
