use crate::sqlx_types::{SqlxError, SqlxRow};
use sea_query::SelectStatement;

#[async_trait::async_trait]
pub trait Connection: Sized + Sync {
    async fn query_all(&self, select: SelectStatement) -> Result<Vec<SqlxRow>, SqlxError>;

    async fn query_all_raw(&self, sql: String) -> Result<Vec<SqlxRow>, SqlxError>;
}
