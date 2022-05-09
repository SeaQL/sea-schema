use sea_schema::sea_query::SqliteQueryBuilder;
use sea_schema::sqlite::discovery::{DiscoveryResult, SchemaDiscovery};
use sqlx::sqlite::SqlitePool;

#[async_std::main]
async fn main() -> DiscoveryResult<()> {
    let sqlite_pool = SqlitePool::connect("sqlite://tests/sakila/sqlite/sakila.db")
        .await
        .unwrap();

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
