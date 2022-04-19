use super::{Statement, StatementBuilder};

/// Database backend
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DatabaseBackend {
    /// MySQL
    MySql,
    /// PostgreSQL
    Postgres,
    /// SQLite
    Sqlite,
}

impl DatabaseBackend {
    /// Build a SeaQuery statement into [`Statement`]
    pub(crate) fn build<S>(&self, statement: &S) -> Statement
    where
        S: StatementBuilder,
    {
        statement.build(self)
    }
}

/// Alias of [`DatabaseBackend`]
pub type DbBackend = DatabaseBackend;
