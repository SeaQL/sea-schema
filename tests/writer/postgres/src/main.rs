use sea_query::PostgresQueryBuilder;
use sea_schema::postgres::discovery::SchemaDiscovery;
use sqlx::{Executor, PgPool, Pool, Postgres};

#[async_std::main]
async fn main() {
    let connection = PgPool::connect("postgres://sea:sea@localhost/sakila")
        .await
        .unwrap();

    let schema_discovery = SchemaDiscovery::new(connection, "public");

    let schema = schema_discovery.discover().await;

    println!("{:#?}", schema);

    let mut connection = setup("postgres://sea:sea@localhost", "sakila_test")
        .await
        .acquire()
        .await
        .unwrap();

    for table in schema.tables.iter() {
        let sql = table.write().to_string(PostgresQueryBuilder);
        let sql = sql.replace("UNIQUE KEY", "UNIQUE");
        println!("{};", sql);
        println!();
        sqlx::query(&sql).execute(&mut connection).await.unwrap();
    }
}

pub async fn setup(base_url: &str, db_name: &str) -> Pool<Postgres> {
    let url = format!("{}/postgres", base_url);
    let mut connection = PgPool::connect(&url)
        .await
        .unwrap()
        .acquire()
        .await
        .unwrap();

    let _drop_db_result = sqlx::query(&format!("DROP DATABASE IF EXISTS \"{}\";", db_name))
        .bind(db_name)
        .execute(&mut connection)
        .await
        .unwrap();

    let _create_db_result = sqlx::query(&format!("CREATE DATABASE \"{}\";", db_name))
        .execute(&mut connection)
        .await
        .unwrap();

    let url = format!("{}/{}", base_url, db_name);
    PgPool::connect(&url).await.unwrap()
}
