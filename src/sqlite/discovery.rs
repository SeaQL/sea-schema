use crate::sqlite::{DiscoveryResult, TableDef};
use sea_query::{Alias, Expr, SelectStatement, SqliteQueryBuilder};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteConnection, SqliteJournalMode, SqliteRow},
    ConnectOptions,
};
use std::str::FromStr;

/// Performs all the methods for schema discovery of a SQLite database
#[derive(Debug)]
pub struct SchemaDiscovery {
    pub database: String,
    pub connection: SqliteConnection,
    pub tables: Vec<TableDef>,
}

impl SchemaDiscovery {
    /// Instantiate a new database connection to the database specified
    ///
    /// ### Usage
    /// ```
    /// SchemaDiscovery::new("foo.db")
    ///     .await?
    /// ```
    pub async fn new(database_name: &str) -> DiscoveryResult<Self> {
        let mut database = String::default();
        database.push_str("sqlite://");
        database.push_str(database_name);

        let sqlite_connection = SqliteConnectOptions::from_str(&database)?
            .journal_mode(SqliteJournalMode::Wal)
            .connect()
            .await?;

        Ok(SchemaDiscovery {
            database: database.to_owned(),
            connection: sqlite_connection,
            tables: Vec::default(),
        })
    }

    /// Discover all the tables in a SQLite database
    pub async fn discover(&mut self) -> DiscoveryResult<&mut Self> {
        let get_tables = SelectStatement::new()
            .column(Alias::new("name"))
            .from(Alias::new("sqlite_master"))
            .and_where(Expr::col(Alias::new("type")).eq("table"))
            .to_string(SqliteQueryBuilder);

        let rows: Vec<SqliteRow> = sqlx::query(&get_tables)
            .fetch_all(&mut self.connection)
            .await?;
        for row in &rows {
            let mut table: TableDef = row.into();
            table.pk_is_autoincrement(&mut self.connection).await?;
            table.get_indexes(&mut self.connection).await?;
            table.get_foreign_keys(&mut self.connection).await?;
            table.get_column_info(&mut self.connection).await?;
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
