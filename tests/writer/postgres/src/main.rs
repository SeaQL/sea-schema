use pretty_assertions::assert_eq;
use sea_schema::{postgres::discovery::SchemaDiscovery, sea_query::PostgresQueryBuilder};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{collections::HashMap, time::Duration};

#[async_std::main]
async fn main() {
    // env_logger::builder()
    //     .filter_level(log::LevelFilter::Debug)
    //     .is_test(true)
    //     .init();
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
        let mut stmts = HashMap::new();
        for table in schema.tables.iter() {
            let stmt = table.write().to_string(PostgresQueryBuilder);
            println!("{};", stmt);
            println!();
            stmts.insert(table.info.name.as_str(), stmt);
        }
        assert_eq!(
            stmts
                .get("artist_release")
                .expect("Create 'artist_release' table statament not found")
                .as_str(),
            [
                r#"CREATE TABLE "artist_release" ("#,
                r#""is_track_artist" bool NOT NULL,"#,
                r#""artist" integer NOT NULL,"#,
                r#""first_release_date" integer,"#,
                r#""catalog_numbers" array,"#,
                r#""country_code" char(2),"#,
                r#""barcode" bigint,"#,
                r#""sort_character" char(1) NOT NULL,"#,
                r#""release" integer NOT NULL,"#,
                r#"CONSTRAINT "artist_release_fk_artist" FOREIGN KEY ("artist") REFERENCES "artist" ("id") ON DELETE CASCADE ON UPDATE NO ACTION,"#,
                r#"CONSTRAINT "artist_release_fk_release" FOREIGN KEY ("release") REFERENCES "release" ("id") ON DELETE CASCADE ON UPDATE NO ACTION"#,
                r#")"#,
            ]
            .join(" ")
            .as_str()
        );
    }
}
