use sea_schema::postgres::discovery::SchemaDiscovery;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

#[async_std::main]
async fn main() {
    {
        // Testing discovery on `sakila` database
        let connection = PgPool::connect("postgres://sea:sea@localhost/sakila")
            .await
            .unwrap();
        let schema_discovery = SchemaDiscovery::new(connection, "public");
        let schema = schema_discovery.discover().await;
        // println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        println!("{:#?}", schema);
    }
    {
        // Testing discovery on `musicbrainz` database
        let connection = PgPoolOptions::new()
            .max_connections(80)
            .connect_timeout(Duration::from_secs(600))
            .connect("postgres://sea:sea@localhost/musicbrainz")
            .await
            .unwrap();
        let schema_discovery = SchemaDiscovery::new(connection, "public");
        let schema = schema_discovery.discover().await;
        println!("{:#?}", schema);
    }
}
