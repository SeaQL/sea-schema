use crate::sqlite::{
    executor::{Executor, IntoExecutor},
    DiscoveryResult, IndexInfo, Schema, TableDef,
};
use sea_query::{Alias, Expr, SelectStatement};
use sqlx::SqlitePool;

/// Performs all the methods for schema discovery of a SQLite database
pub struct SchemaDiscovery {
    pub executor: Executor,
}

impl SchemaDiscovery {
    /// Instantiate a new database connection to the database specified
    ///
    /// ### Usage
    /// ```
    /// SchemaDiscovery::new(sqlite_pool)
    ///     .await?
    /// ```
    pub fn new(sqlite_pool: SqlitePool) -> Self {
        SchemaDiscovery {
            executor: sqlite_pool.into_executor(),
        }
    }

    /// Discover all the tables in a SQLite database
    pub async fn discover(&self) -> DiscoveryResult<Schema> {
        let get_tables = SelectStatement::new()
            .column(Alias::new("name"))
            .from(Alias::new("sqlite_master"))
            .and_where(Expr::col(Alias::new("type")).eq("table"))
            .and_where(Expr::col(Alias::new("name")).ne("sqlite_sequence"))
            .to_owned();

        let mut tables = Vec::new();
        for row in self.executor.fetch_all(get_tables).await {
            let mut table: TableDef = (&row).into();
            table.pk_is_autoincrement(&self.executor).await?;
            table.get_foreign_keys(&self.executor).await?;
            table.get_column_info(&self.executor).await?;
            tables.push(table);
        }

        Ok(Schema { tables })
    }

    /// Discover table indexes
    pub async fn discover_indexes(&self) -> DiscoveryResult<Vec<IndexInfo>> {
        let get_tables = SelectStatement::new()
            .column(Alias::new("name"))
            .from(Alias::new("sqlite_master"))
            .and_where(Expr::col(Alias::new("type")).eq("table"))
            .and_where(Expr::col(Alias::new("name")).ne("sqlite_sequence"))
            .to_owned();

        let mut tables = Vec::new();
        let rows = self.executor.fetch_all(get_tables).await;
        for row in rows {
            let table: TableDef = (&row).into();
            tables.push(table);
        }

        let mut discovered_indexes: Vec<IndexInfo> = Vec::default();

        for table in tables.iter_mut() {
            table
                .get_indexes(&self.executor, &mut discovered_indexes)
                .await?
        }

        Ok(discovered_indexes)
    }
}
