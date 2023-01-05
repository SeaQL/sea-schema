use sea_schema::sea_query::SqliteQueryBuilder;
use sea_schema::sqlite::discovery::{DiscoveryResult, SchemaDiscovery};
use sqlx::sqlite::SqlitePool;

#[async_std::main]
async fn main() -> DiscoveryResult<()> {
    let url = std::env::var("DATABASE_URL_SAKILA")
        .unwrap_or_else(|_| "sqlite://tests/sakila/sqlite/sakila.db".to_owned());

    let sqlite_pool = SqlitePool::connect(&url).await.unwrap();

    let schema_discovery = SchemaDiscovery::new(sqlite_pool);

    let discover_tables = schema_discovery.discover().await?;

    for table in discover_tables.tables.iter() {
        println!("{};", table.write().to_string(SqliteQueryBuilder));
    }

    let discover_indexes = schema_discovery.discover_indexes().await?;

    for index in discover_indexes.iter() {
        println!("{};", index.write().to_string(SqliteQueryBuilder));
    }

    Ok(())
}
