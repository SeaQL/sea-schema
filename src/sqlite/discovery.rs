use sea_query::{Expr, ExprTrait, SelectStatement};

use super::def::{IndexInfo, Schema, TableDef};
pub use super::error::DiscoveryResult;
use super::executor::{Executor, IntoExecutor};
use super::query::SqliteMaster;
use crate::{Connection, sqlx_types::SqlitePool};

/// Performs all the methods for schema discovery of a SQLite database
pub struct SchemaDiscovery<C: Connection> {
    conn: C,
}

impl SchemaDiscovery<Executor> {
    /// Discover schema from a SQLx pool
    pub fn new(pool: SqlitePool) -> Self {
        Self::conn(pool.into_executor())
    }
}

impl<C: Connection> SchemaDiscovery<C> {
    /// Discover schema from a generic SQLx connection
    pub fn conn(conn: C) -> Self {
        Self { conn }
    }

    /// Discover all the tables in a SQLite database
    pub async fn discover(&self) -> DiscoveryResult<Schema> {
        let get_tables = SelectStatement::new()
            .column("name")
            .from(SqliteMaster)
            .and_where(Expr::col("type").eq("table"))
            .and_where(Expr::col("name").ne("sqlite_sequence"))
            .to_owned();

        let mut tables = Vec::new();
        for row in self.conn.query_all(get_tables).await? {
            let mut table: TableDef = row.into();
            table.pk_is_autoincrement(&self.conn).await?;
            table.get_foreign_keys(&self.conn).await?;
            table.get_column_info(&self.conn).await?;
            table.get_constraints(&self.conn).await?;
            tables.push(table);
        }

        let indexes = self.discover_indexes().await?;

        Ok(Schema { tables, indexes })
    }

    /// Discover table indexes
    pub async fn discover_indexes(&self) -> DiscoveryResult<Vec<IndexInfo>> {
        let get_tables = SelectStatement::new()
            .column("name")
            .from(SqliteMaster)
            .and_where(Expr::col("type").eq("table"))
            .and_where(Expr::col("name").ne("sqlite_sequence"))
            .to_owned();

        let mut discovered_indexes = Vec::new();
        let rows = self.conn.query_all(get_tables).await?;
        for row in rows {
            let mut table: TableDef = row.into();
            table.get_indexes(&self.conn).await?;
            discovered_indexes.append(&mut table.indexes);
        }

        Ok(discovered_indexes)
    }
}
