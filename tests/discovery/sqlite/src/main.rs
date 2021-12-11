use sea_schema::sea_query::SqliteQueryBuilder;
use sea_schema::sqlite::{DiscoveryResult, SchemaDiscovery};
use sqlx::SqlitePool;
#[async_std::main]
async fn main() -> DiscoveryResult<()> {
    let connection = SqlitePool::connect("sqlite://tests/sakila/sqlite/sakila.db")
        .await
        .unwrap();

    let mut schema_discovery = SchemaDiscovery::new(connection);

    let schema = schema_discovery.discover().await?;

    // println!("{}", serde_json::to_string_pretty(&schema).unwrap());

    println!("{:#?}", schema);

    Ok(())
}
