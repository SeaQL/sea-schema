use sea_query::{Expr, ExprTrait, SelectStatement};

use super::def::{IndexInfo, Schema, TableDef};
pub use super::error::DiscoveryResult;
use super::executor::{Executor, IntoExecutor};
use super::query::SqliteMaster;
use crate::{Connection, sqlx_types::SqlitePool};

/// Performs all the methods for schema discovery of a SQLite database
pub struct SchemaDiscovery {
    exec: Executor,
}

impl SchemaDiscovery {
    /// Discover schema from a SQLx pool
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            exec: pool.into_executor(),
        }
    }

    pub async fn discover(&self) -> DiscoveryResult<Schema> {
        Self::discover_with(&self.exec).await
    }
}

impl SchemaDiscovery {
    /// Discover all the tables in a SQLite database
    pub async fn discover_with<C: Connection>(conn: &C) -> DiscoveryResult<Schema> {
        let get_tables = SelectStatement::new()
            .column("name")
            .from(SqliteMaster)
            .and_where(Expr::col("type").eq("table"))
            .and_where(Expr::col("name").ne("sqlite_sequence"))
            .to_owned();

        let mut tables = Vec::new();
        for row in conn.query_all(get_tables).await? {
            let mut table: TableDef = row.into();
            table.pk_is_autoincrement(conn).await?;
            table.get_foreign_keys(conn).await?;
            table.get_column_info(conn).await?;
            table.get_constraints(conn).await?;
            tables.push(table);
        }

        let indexes = Self::discover_indexes(conn).await?;

        Ok(Schema { tables, indexes })
    }

    /// Discover table indexes
    async fn discover_indexes<C: Connection>(conn: &C) -> DiscoveryResult<Vec<IndexInfo>> {
        let get_tables = SelectStatement::new()
            .column("name")
            .from(SqliteMaster)
            .and_where(Expr::col("type").eq("table"))
            .and_where(Expr::col("name").ne("sqlite_sequence"))
            .to_owned();

        let mut discovered_indexes = Vec::new();
        let rows = conn.query_all(get_tables).await?;
        for row in rows {
            let mut table: TableDef = row.into();
            table.get_indexes(conn).await?;
            discovered_indexes.append(&mut table.indexes);
        }

        Ok(discovered_indexes)
    }
}
