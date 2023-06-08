use sea_schema::postgres::discovery::SchemaDiscovery;
use sea_schema::sea_query::PostgresQueryBuilder;
use sqlx::PgPool;

#[async_std::main]
async fn main() {
    // env_logger::builder()
    //     .filter_level(log::LevelFilter::Debug)
    //     .is_test(true)
    //     .init();

    let url = std::env::var("DATABASE_URL_SAKILA")
        .unwrap_or_else(|_| "postgres://root:root@localhost/sakila".to_owned());

    let connection = PgPool::connect(&url).await.unwrap();

    let schema_discovery = SchemaDiscovery::new(connection, "public");

    let schema = schema_discovery
        .discover()
        .await
        .expect("Error discovering schema");

    for table in schema.tables.iter() {
        println!("{};", table.write().to_string(PostgresQueryBuilder));
        println!();
    }
}
