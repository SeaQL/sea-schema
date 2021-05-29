use sea_query::MysqlQueryBuilder;
use sea_schema::mysql::discovery::SchemaDiscovery;
use sqlx::MySqlPool;

#[async_std::main]
async fn main() {
    let connection = MySqlPool::connect("mysql://sea:sea@localhost/sakila")
        .await
        .unwrap();

    let schema_discovery = SchemaDiscovery::new(connection, "sakila");

    let schema = schema_discovery.discover().await;

    for table in schema.tables.iter() {
        println!("{};", table.write().to_string(MysqlQueryBuilder));
        println!();
    }
}
