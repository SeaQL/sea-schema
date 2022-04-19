use super::{DbBackend, MigrationErr, QueryResultTrait, Statement};

/// Database connection for database migration
#[async_trait::async_trait]
pub trait ConnectionTrait: Send + Sync {
    /// Database backend of current connection
    fn get_database_backend(&self) -> DbBackend;

    /// Execute a SQL statement on the database
    async fn execute(&self, stmt: Statement) -> Result<(), MigrationErr>;

    /// Query single row of result from the database
    async fn query_one(
        &self,
        stmt: Statement,
    ) -> Result<Option<Box<dyn QueryResultTrait>>, MigrationErr>;

    /// Query multiple rows of result from the database
    async fn query_all(
        &self,
        stmt: Statement,
    ) -> Result<Vec<Box<dyn QueryResultTrait>>, MigrationErr>;
}
