use super::MigrationErr;

/// Get column value from the query result
pub trait QueryResultTrait {
    /// Get [`String`] from the query result
    fn try_get_string(&self, col: &str) -> Result<String, MigrationErr>;

    /// Get [`i64`] from the query result
    fn try_get_i64(&self, col: &str) -> Result<i64, MigrationErr>;
}
