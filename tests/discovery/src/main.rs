use sea_query::Alias;
use sea_schema::mysql::discovery::SchemaDiscovery;
use sqlx::MySqlPool;

#[async_std::main]
async fn main() {

    let connection = MySqlPool::connect("mysql://sea:sea@localhost/sakila").await.unwrap();

    let schema_discovery = SchemaDiscovery::new(connection, Alias::new("sakila"));

    let schema = schema_discovery.discover().await;

    // println!("{}", serde_json::to_string_pretty(&schema).unwrap());

    println!("{:#?}", schema);
}
