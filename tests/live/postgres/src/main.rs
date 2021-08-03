use std::collections::HashMap;

use sea_query::{
    Alias, ColumnDef, ForeignKey, Index, PostgresQueryBuilder, Table, TableCreateStatement,
};
use sea_schema::postgres::{def::TableDef, discovery::SchemaDiscovery};
use sqlx::{Executor, PgPool, Pool, Postgres};

#[cfg_attr(test, async_std::test)]
#[cfg_attr(not(test), async_std::main)]
async fn main() {
    let mut connection = setup("postgres://sea:sea@localhost", "sea-schema").await;
    let mut executor = connection.acquire().await.unwrap();

    let tables = vec![
        ("actor", create_actor_table()),
        ("film", create_film_table()),
        ("film_actor", create_film_actor_table()),
    ];

    for (_, tbl_create_stmt) in tables.iter() {
        let sql = tbl_create_stmt.to_string(PostgresQueryBuilder);
        let sql = replace_sql(sql);
        println!("{};", sql);
        println!();
        sqlx::query(&sql).execute(&mut executor).await.unwrap();
    }

    let schema_discovery = SchemaDiscovery::new(connection, "public");

    let schema = schema_discovery.discover().await;

    let map: HashMap<String, TableDef> = schema
        .tables
        .iter()
        .map(|table| (table.info.name.clone(), table.clone()))
        .collect();

    for (table, tbl_create_stmt) in tables.into_iter() {
        let expected_sql = tbl_create_stmt.to_string(PostgresQueryBuilder);
        let expected_sql = replace_sql(expected_sql);
        let table = map.get(table).unwrap();
        let sql = table.write().to_string(PostgresQueryBuilder);
        let sql = replace_sql(sql);
        println!("Expected SQL:");
        println!("{};", expected_sql);
        println!("Generated SQL:");
        println!("{};", sql);
        println!();
        assert_eq!(expected_sql, sql);
    }
}

pub fn replace_sql(sql: String) -> String {
    sql.replace("UNIQUE KEY", "UNIQUE")
        .replace("CONSTRAINT FOREIGN KEY", "FOREIGN KEY")
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

pub fn create_actor_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("actor"))
        .col(
            ColumnDef::new(Alias::new("actor_id"))
                .integer()
                .auto_increment()
                .not_null(),
        )
        .col(
            ColumnDef::new(Alias::new("first_name"))
                .char_len(45)
                .not_null(),
        )
        .col(
            ColumnDef::new(Alias::new("last_name"))
                .char_len(45)
                .not_null(),
        )
        .col(
            ColumnDef::new(Alias::new("last_update"))
                .timestamp_len(6)
                .not_null()
                .extra("DEFAULT now()".to_owned()),
        )
        .primary_key(Index::create().primary().col(Alias::new("actor_id")))
        .to_owned()
}

pub fn create_film_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("film"))
        .col(
            ColumnDef::new(Alias::new("film_id"))
                .integer()
                .auto_increment()
                .not_null(),
        )
        .col(ColumnDef::new(Alias::new("title")).char_len(255).not_null())
        .col(ColumnDef::new(Alias::new("description")).text())
        .col(
            ColumnDef::new(Alias::new("rental_rate"))
                .decimal_len(4, 2)
                .not_null()
                .default(4.99),
        )
        .col(
            ColumnDef::new(Alias::new("last_update"))
                .timestamp_len(6)
                .not_null()
                .extra("DEFAULT now()".to_owned()),
        )
        .primary_key(Index::create().primary().col(Alias::new("film_id")))
        .index(Index::create().unique().col(Alias::new("title")))
        .to_owned()
}

pub fn create_film_actor_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("film_actor"))
        .col(ColumnDef::new(Alias::new("actor_id")).integer().not_null())
        .col(ColumnDef::new(Alias::new("film_id")).integer().not_null())
        .col(
            ColumnDef::new(Alias::new("last_update"))
                .timestamp_len(6)
                .not_null()
                .extra("DEFAULT now()".to_owned()),
        )
        .primary_key(
            Index::create()
                .primary()
                .col(Alias::new("actor_id"))
                .col(Alias::new("film_id")),
        )
        .foreign_key(
            ForeignKey::create()
                .to_tbl(Alias::new("actor"))
                .from_col(Alias::new("actor_id"))
                .to_col(Alias::new("actor_id")),
        )
        .foreign_key(
            ForeignKey::create()
                .to_tbl(Alias::new("film"))
                .from_col(Alias::new("film_id"))
                .to_col(Alias::new("film_id")),
        )
        .to_owned()
}
