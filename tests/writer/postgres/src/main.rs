use sea_schema::{postgres::discovery::SchemaDiscovery, sea_query::PostgresQueryBuilder};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

#[async_std::main]
async fn main() {
    {
        // Testing writer on `sakila` database
        let connection = PgPool::connect("postgres://sea:sea@localhost/sakila")
            .await
            .unwrap();
        let schema_discovery = SchemaDiscovery::new(connection, "public");
        let schema = schema_discovery.discover().await;
        for table in schema.tables.iter() {
            println!("{};", table.write().to_string(PostgresQueryBuilder));
            println!();
        }
    }
    {
        // Testing writer on `musicbrainz` database
        let connection = PgPoolOptions::new()
            .max_connections(80)
            .connect_timeout(Duration::from_secs(600))
            .connect("postgres://sea:sea@localhost/musicbrainz")
            .await
            .unwrap();
        let schema_discovery = SchemaDiscovery::new(connection, "public");
        let schema = schema_discovery.discover().await;
        for table in schema.tables.iter() {
            println!("{};", table.write().to_string(PostgresQueryBuilder));
            println!();
        }
    }
}
