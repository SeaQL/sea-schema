use sea_schema::sqlite::{SchemaDiscovery, DiscoveryResult};
use sea_schema::sea_query::SqliteQueryBuilder;

#[async_std::main]
async fn main() -> DiscoveryResult<()> {
    // let connection = SqlitePool::connect("sqlite://tests/sakila/sqlite/sakila.db")
    //     .await
    //     .unwrap();

    let mut schema_discovery = SchemaDiscovery::new("tests/sakila/sqlite/sakila.db").await?;

    let schema = schema_discovery.discover().await?;

    for table in schema.tables.iter_mut() {
        // println!("{};", table.write().to_string(SqliteQueryBuilder));
        println!("{};", table.to_sql_statement());
        println!("{};", table.to_sql_statement_with_indexes());
        println!();
    }

    Ok(())
}
