use sea_query::PostgresQueryBuilder;
use sea_schema::postgres::discovery::SchemaDiscovery;
use sqlx::PgPool;

#[async_std::main]
async fn main() {
    let connection = PgPool::connect("postgres://sea:sea@localhost/sakila")
        .await
        .unwrap();

    let schema_discovery = SchemaDiscovery::new(connection, "public");

    let schema = schema_discovery.discover().await;

    println!("{:#?}", schema);

    for table in schema.tables.iter() {
        println!("{};", table.write().to_string(PostgresQueryBuilder));
        println!();
    }
}
