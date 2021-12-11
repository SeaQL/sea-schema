use sea_schema::mysql::discovery::SchemaDiscovery;
use sqlx::MySqlPool;

#[async_std::main]
async fn main() {
    // env_logger::builder()
    //     .filter_level(log::LevelFilter::Debug)
    //     .is_test(true)
    //     .init();

    let connection = MySqlPool::connect("mysql://sea:sea@localhost/sakila")
        .await
        .unwrap();

    let schema_discovery = SchemaDiscovery::new(connection, "sakila");

    let schema = schema_discovery.discover().await;

    // println!("{}", serde_json::to_string_pretty(&schema).unwrap());

    println!("{:#?}", schema);
}
