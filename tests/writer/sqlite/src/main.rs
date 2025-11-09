use sea_schema::sea_query::SqliteQueryBuilder;
use sea_schema::sqlite::discovery::{DiscoveryResult, SchemaDiscovery};
use sqlx::sqlite::SqlitePool;

#[async_std::main]
async fn main() -> DiscoveryResult<()> {
    let url = std::env::var("DATABASE_URL_SAKILA")
        .unwrap_or_else(|_| "sqlite://tests/sakila/sqlite/sakila.db".to_owned());

    let sqlite_pool = SqlitePool::connect(&url).await.unwrap();

    let schema_discovery = SchemaDiscovery::new(sqlite_pool);

    let schema = schema_discovery.discover().await?;

    for table in schema.tables.iter() {
        println!("{};", table.write().to_string(SqliteQueryBuilder));
    }

    for index in schema.indexes.iter() {
        println!("{};", index.write().to_string(SqliteQueryBuilder));
    }

    Ok(())
}
