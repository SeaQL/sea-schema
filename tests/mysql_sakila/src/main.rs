use async_std::task;
use sea_query::Alias;
use sea_schema::mysql::schema::SchemaDiscovery;
use sqlx::MySqlPool;

fn main() {

    let connection = task::block_on(async {
        MySqlPool::connect("mysql://sea:sea@localhost/sakila").await.unwrap()
    });

    let schema_discovery = SchemaDiscovery::new(connection, Alias::new("sakila"));

    let schema = task::block_on(async {
        schema_discovery.discover().await
    });

    println!("{:#?}", schema);
}
