use crate::sqlite::{DiscoveryResult, TableDef};
use sea_query::{Alias, Expr, SelectStatement, SqliteQueryBuilder};
use sqlx::sqlite::{SqlitePool, SqliteRow};

/// Performs all the methods for schema discovery of a SQLite database
#[derive(Debug)]
pub struct SchemaDiscovery {
    pub pool: SqlitePool,
    pub tables: Vec<TableDef>,
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
            pool: sqlite_pool,
            tables: Vec::default(),
        }
    }

    /// Discover all the tables in a SQLite database
    pub async fn discover(&mut self) -> DiscoveryResult<&mut Self> {
        let get_tables = SelectStatement::new()
            .column(Alias::new("name"))
            .from(Alias::new("sqlite_master"))
            .and_where(Expr::col(Alias::new("type")).eq("table"))
            .to_string(SqliteQueryBuilder);

        let rows: Vec<SqliteRow> = sqlx::query(&get_tables)
            .fetch_all(&mut self.pool.acquire().await?)
            .await?;
        for row in &rows {
            let mut table: TableDef = row.into();
            table.pk_is_autoincrement(&mut self.pool).await?;
            table.get_indexes(&mut self.pool).await?;
            table.get_foreign_keys(&mut self.pool).await?;
            table.get_column_info(&mut self.pool).await?;
            self.tables.push(table);
        }

        Ok(self)
    }

    /// Map all the discovered tables into an sqlite statement
    pub fn to_sql(&self) -> Vec<String> {
        let statements = self
            .tables
            .iter()
            .map(|table| table.to_sql_statement())
            .collect::<Vec<String>>();

        statements
    }
}
