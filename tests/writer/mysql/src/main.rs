use sea_schema::mysql::discovery::SchemaDiscovery;
use sea_schema::sea_query::MysqlQueryBuilder;
use sqlx::MySqlPool;

#[async_std::main]
async fn main() {
    // env_logger::builder()
    //     .filter_level(log::LevelFilter::Debug)
    //     .is_test(true)
    //     .init();

    let url = std::env::var("DATABASE_URL_SAKILA")
        .unwrap_or_else(|_| "mysql://root:root@localhost".to_owned());

    let connection = MySqlPool::connect(&url).await.unwrap();

    let schema_discovery = SchemaDiscovery::new(connection, "sakila");

    let schema = schema_discovery
        .discover()
        .await
        .expect("Error discovering schema");

    for table in schema.tables.iter() {
        println!("{};", table.write().to_string(MysqlQueryBuilder));
        println!();
    }
}
