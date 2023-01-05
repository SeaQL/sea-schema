use sea_schema::sqlite::discovery::{DiscoveryResult, SchemaDiscovery};
use sqlx::SqlitePool;

#[async_std::main]
async fn main() -> DiscoveryResult<()> {
    let url = std::env::var("DATABASE_URL_SAKILA")
        .unwrap_or_else(|_| "sqlite://tests/sakila/sqlite/sakila.db".to_owned());

    let connection = SqlitePool::connect(&url).await.unwrap();

    let schema_discovery = SchemaDiscovery::new(connection);

    let schema = schema_discovery.discover().await?;

    // println!("{}", serde_json::to_string_pretty(&schema).unwrap());

    println!("{:#?}", schema);

    Ok(())
}
