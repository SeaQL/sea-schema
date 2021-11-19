use std::collections::HashMap;

use sea_schema::postgres::{def::TableDef, discovery::SchemaDiscovery};
use sea_schema::sea_query::{
    Alias, ColumnDef, ForeignKey, ForeignKeyAction, Index, PostgresQueryBuilder, Table,
    TableCreateStatement,
};
use sqlx::{PgPool, Pool, Postgres};

#[cfg_attr(test, async_std::test)]
#[cfg_attr(not(test), async_std::main)]
async fn main() {
    let connection = setup("postgres://sea:sea@localhost", "sea-schema").await;
    let mut executor = connection.acquire().await.unwrap();

    let tbl_create_stmts = vec![
        create_bakery_table(),
        create_baker_table(),
        create_customer_table(),
        create_order_table(),
        create_cake_table(),
        create_cakes_bakers_table(),
        create_lineitem_table(),
    ];

    for tbl_create_stmt in tbl_create_stmts.iter() {
        let sql = tbl_create_stmt.to_string(PostgresQueryBuilder);
        println!("{};", sql);
        println!();
        sqlx::query(&sql).execute(&mut executor).await.unwrap();
    }

    let schema_discovery = SchemaDiscovery::new(connection, "public");

    let schema = schema_discovery.discover().await;

    println!("{:#?}", schema);

    let map: HashMap<String, TableDef> = schema
        .tables
        .iter()
        .map(|table| (table.info.name.clone(), table.clone()))
        .collect();

    for tbl_create_stmt in tbl_create_stmts.into_iter() {
        let expected_sql = tbl_create_stmt.to_string(PostgresQueryBuilder);
        let table = map.get(&tbl_create_stmt.get_table_name().unwrap()).unwrap();
        let sql = table.write().to_string(PostgresQueryBuilder);
        println!("Expected SQL:");
        println!("{};", expected_sql);
        println!("Generated SQL:");
        println!("{};", sql);
        println!();
        assert_eq!(expected_sql, sql);
    }

    create_fetch_verify_enum().await;
}

async fn setup(base_url: &str, db_name: &str) -> Pool<Postgres> {
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

fn create_bakery_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("bakery"))
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment(),
        )
        .col(ColumnDef::new(Alias::new("name")).string())
        .col(ColumnDef::new(Alias::new("profit_margin")).double())
        .primary_key(
            Index::create()
                .primary()
                .name("bakery_pkey")
                .col(Alias::new("id")),
        )
        .to_owned()
}

fn create_baker_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("baker"))
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment(),
        )
        .col(ColumnDef::new(Alias::new("name")).string())
        .col(ColumnDef::new(Alias::new("contact_details")).json())
        .col(ColumnDef::new(Alias::new("bakery_id")).integer())
        .primary_key(
            Index::create()
                .primary()
                .name("baker_pkey")
                .col(Alias::new("id")),
        )
        .foreign_key(
            ForeignKey::create()
                .name("FK_baker_bakery")
                .from(Alias::new("baker"), Alias::new("bakery_id"))
                .to(Alias::new("bakery"), Alias::new("id"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_customer_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("customer"))
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment(),
        )
        .col(ColumnDef::new(Alias::new("name")).string())
        .col(ColumnDef::new(Alias::new("notes")).text())
        .primary_key(
            Index::create()
                .primary()
                .name("customer_pkey")
                .col(Alias::new("id")),
        )
        .to_owned()
}

fn create_order_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("order"))
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment(),
        )
        .col(ColumnDef::new(Alias::new("total")).decimal_len(19, 4))
        .col(ColumnDef::new(Alias::new("bakery_id")).integer().not_null())
        .col(
            ColumnDef::new(Alias::new("customer_id"))
                .integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(Alias::new("placed_at"))
                .timestamp_len(6)
                .not_null(),
        )
        .primary_key(
            Index::create()
                .primary()
                .name("order_pkey")
                .col(Alias::new("id")),
        )
        .foreign_key(
            ForeignKey::create()
                .name("FK_order_bakery")
                .from(Alias::new("order"), Alias::new("bakery_id"))
                .to(Alias::new("bakery"), Alias::new("id"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .foreign_key(
            ForeignKey::create()
                .name("FK_order_customer")
                .from(Alias::new("order"), Alias::new("customer_id"))
                .to(Alias::new("customer"), Alias::new("id"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_lineitem_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("lineitem"))
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment(),
        )
        .col(ColumnDef::new(Alias::new("price")).decimal_len(19, 4))
        .col(ColumnDef::new(Alias::new("quantity")).integer())
        .col(ColumnDef::new(Alias::new("order_id")).integer().not_null())
        .col(ColumnDef::new(Alias::new("cake_id")).integer().not_null())
        .primary_key(
            Index::create()
                .primary()
                .name("lineitem_pkey")
                .col(Alias::new("id")),
        )
        .foreign_key(
            ForeignKey::create()
                .name("FK_lineitem_cake")
                .from(Alias::new("lineitem"), Alias::new("cake_id"))
                .to(Alias::new("cake"), Alias::new("id"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .foreign_key(
            ForeignKey::create()
                .name("FK_lineitem_order")
                .from(Alias::new("lineitem"), Alias::new("order_id"))
                .to(Alias::new("order"), Alias::new("id"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_cakes_bakers_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("cakes_bakers"))
        .col(ColumnDef::new(Alias::new("cake_id")).integer().not_null())
        .col(ColumnDef::new(Alias::new("baker_id")).integer().not_null())
        .primary_key(
            Index::create()
                .name("cakes_bakers_pkey")
                .col(Alias::new("cake_id"))
                .col(Alias::new("baker_id")),
        )
        .to_owned()
}

fn create_cake_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("cake"))
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment(),
        )
        .col(ColumnDef::new(Alias::new("name")).string())
        .col(ColumnDef::new(Alias::new("price")).decimal_len(19, 4))
        .col(ColumnDef::new(Alias::new("bakery_id")).integer().not_null())
        .col(ColumnDef::new(Alias::new("gluten_free")).boolean())
        .col(ColumnDef::new(Alias::new("serial")).uuid())
        .primary_key(
            Index::create()
                .primary()
                .name("cake_pkey")
                .col(Alias::new("id")),
        )
        .foreign_key(
            ForeignKey::create()
                .name("FK_cake_bakery")
                .from(Alias::new("cake"), Alias::new("bakery_id"))
                .to(Alias::new("bakery"), Alias::new("id"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

/// A test case for PostgreSQL enums in the following steps:
///
/// Creates an enum with string types that have special characters
/// Add the enum type to the database
/// Discover all enums from the database
/// Assert that the enum created earlier is amongst those discovered
async fn create_fetch_verify_enum() {
    use sea_query::{extension::postgres::TypeCreateStatement, Alias, PostgresQueryBuilder};

    let create_crazy_enum = TypeCreateStatement::new()
        .as_enum(Alias::new("crazy_enum"))
        .values(vec![
            Alias::new("Astro0%00%8987,.!@#$%^&*()_-+=[]{}\\|.<>/? ``"),
            Alias::new("Biology"),
            Alias::new("Chemistry"),
            Alias::new("Math"),
            Alias::new("Physics"),
        ])
        .to_string(PostgresQueryBuilder);

    use sqlx::postgres::PgPoolOptions;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://sea:sea@localhost/sakila")
        .await
        .unwrap();

    sqlx::query(&create_crazy_enum)
        .execute(&pool)
        .await
        .unwrap();

    let connection = PgPool::connect("postgres://sea:sea@localhost/sakila")
        .await
        .unwrap();

    let enums_discovery = SchemaDiscovery::new(connection, "public")
        .discover_enums()
        .await;

    dbg!(&enums_discovery);

    let enum_create_statements = enums_discovery
        .into_iter()
        .map(|enum_type| enum_type.get_enum_def().to_sql_query())
        .collect::<Vec<String>>();

    dbg!(&enum_create_statements);

    assert!(
        match enum_create_statements.binary_search(&create_crazy_enum) {
            Ok(_) => true,
            Err(_) => false,
        }
    );
}
