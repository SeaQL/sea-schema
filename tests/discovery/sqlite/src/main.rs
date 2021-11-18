use sea_schema::sqlite::{SchemaDiscovery, DiscoveryResult};
use sea_schema::sea_query::SqliteQueryBuilder;

#[async_std::main]
async fn main() -> DiscoveryResult<()> {
    // let connection = SqlitePool::connect("sqlite://tests/sakila/sqlite/sakila.db")
    //     .await
    //     .unwrap();

    let mut schema_discovery = SchemaDiscovery::new("tests/sakila/sqlite/sakila.db").await?;

    let schema = schema_discovery.discover().await?;

    // println!("{}", serde_json::to_string_pretty(&schema).unwrap());

    println!("{:#?}", schema);

    Ok(())
}
