use crate::sqlx_types::{SqlxError, SqlxRow};
use sea_query::SelectStatement;

#[cfg_attr(feature = "sqlx-dep", async_trait::async_trait)]
#[cfg_attr(not(feature = "sqlx-dep"), async_trait::async_trait(?Send))]
pub trait Connection: Sized + Sync {
    async fn query_all(&self, select: SelectStatement) -> Result<Vec<SqlxRow>, SqlxError>;

    async fn query_all_raw(&self, sql: String) -> Result<Vec<SqlxRow>, SqlxError>;
}
