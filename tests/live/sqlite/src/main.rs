use pretty_assertions::assert_eq;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::collections::HashMap;

use sea_schema::sea_query::{
    Alias, ColumnDef, ForeignKey, ForeignKeyAction, ForeignKeyCreateStatement, Index, Query,
    SqliteQueryBuilder, Table, TableCreateStatement, TableRef,
};
use sea_schema::sqlite::{
    def::TableDef,
    discovery::{DiscoveryResult, SchemaDiscovery},
};

#[cfg_attr(test, async_std::test)]
#[cfg_attr(not(test), async_std::main)]
async fn main() -> DiscoveryResult<()> {
    // env_logger::builder()
    //     .filter_level(log::LevelFilter::Debug)
    //     .is_test(true)
    //     .init();

    test_001().await?;
    test_002().await?;

    Ok(())
}

async fn test_001() -> DiscoveryResult<()> {
    let sqlite_pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    //DROP TABLES to ensure all tests pass
    sqlx::query("DROP TABLE IF EXISTS Programming_Langs")
        .fetch_all(&mut sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query("DROP TABLE IF EXISTS suppliers")
        .fetch_all(&mut sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query("DROP TABLE IF EXISTS supplier_groups")
        .fetch_all(&mut sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    // Creates a new test table
    let create_table = Table::create()
        .table(Alias::new("Programming_Langs"))
        .col(
            ColumnDef::new(Alias::new("Name"))
                .custom(Alias::new("INTEGER"))
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(Alias::new("SLOC"))
                .custom(Alias::new("INT8"))
                .not_null()
                .default(100.45),
        )
        .col(
            ColumnDef::new(Alias::new("SemVer"))
                .custom(Alias::new("VARCHAR(255)"))
                .not_null(),
        )
        .col(
            ColumnDef::new(Alias::new("last_update"))
                .custom(Alias::new("DATETIME"))
                .not_null()
                .default("1990-01-01 00:00:00"),
        )
        .to_owned();

    // This ensures that the `sqlite_sequence` table is populated

    let insert_into_table = Query::insert()
        .into_table(Alias::new("Programming_Langs"))
        .columns(vec![Alias::new("SLOC"), Alias::new("SemVer")])
        .values(vec![4.into(), "0.1.0".into()])
        .unwrap()
        .to_owned();

    let create_index = Index::create()
        .name("idx-programming_lang-aspect")
        .table(Alias::new("Programming_Langs"))
        .col(Alias::new("SLOC"))
        .col(Alias::new("SemVer"))
        .to_owned();

    //DROP TABLES to ensure all tests pass
    sqlx::query("DROP TABLE IF EXISTS Programming_Langs")
        .fetch_all(&mut sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query("DROP TABLE IF EXISTS suppliers")
        .fetch_all(&mut sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query("DROP TABLE IF EXISTS supplier_groups")
        .fetch_all(&mut sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    // Tests foreign key discovery
    let table_create_suppliers = Table::create()
        .table(Alias::new("suppliers"))
        .col(ColumnDef::new(Alias::new("supplier_id")).custom(Alias::new("INTEGER")))
        .col(
            ColumnDef::new(Alias::new("supplier_name"))
                .custom(Alias::new("TEXT"))
                .not_null(),
        )
        .col(
            ColumnDef::new(Alias::new("group_id"))
                .custom(Alias::new("INTEGER"))
                .not_null(),
        )
        .primary_key(Index::create().col(Alias::new("supplier_id")))
        .foreign_key(
            ForeignKeyCreateStatement::new()
                .name("group_id")
                .from(Alias::new("suppliers"), Alias::new("group_id"))
                .to(Alias::new("supplier_groups"), Alias::new("group_id"))
                .on_delete(ForeignKeyAction::SetNull)
                .on_update(ForeignKeyAction::SetNull),
        )
        .to_owned();

    let table_create_supplier_groups = Table::create()
        .table(Alias::new("supplier_groups"))
        .col(ColumnDef::new(Alias::new("group_id")).custom(Alias::new("INTEGER")))
        .col(
            ColumnDef::new(Alias::new("group_name"))
                .custom(Alias::new("TEXT"))
                .not_null(),
        )
        .primary_key(Index::create().col(Alias::new("group_id")))
        .to_owned();

    println!(
        "{:?}",
        &table_create_suppliers.to_string(SqliteQueryBuilder)
    );

    let insert_into_supplier_groups = Query::insert()
        .into_table(Alias::new("supplier_groups"))
        .columns(vec![Alias::new("group_name")])
        .values(vec!["Global".into()])
        .unwrap()
        .to_owned();

    let insert_into_suppliers = Query::insert()
        .into_table(Alias::new("suppliers"))
        .columns(vec![Alias::new("supplier_name"), Alias::new("group_id")])
        .values(vec!["HP".into(), 1.into()])
        .unwrap()
        .to_owned();

    sqlx::query(&create_table.to_string(SqliteQueryBuilder))
        .fetch_all(&mut sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query(&insert_into_table.to_string(SqliteQueryBuilder))
        .fetch_all(&mut sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query(&table_create_supplier_groups.to_string(SqliteQueryBuilder))
        .fetch_all(&mut sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query(&table_create_suppliers.to_string(SqliteQueryBuilder))
        .fetch_all(&mut sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query(&insert_into_supplier_groups.to_string(SqliteQueryBuilder))
        .fetch_all(&mut sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query(&insert_into_suppliers.to_string(SqliteQueryBuilder))
        .fetch_all(&mut sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query(&create_index.to_string(SqliteQueryBuilder))
        .fetch_all(&mut sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    let discovered_schema = SchemaDiscovery::new(sqlite_pool.clone());

    let convert_column_types = |str: String| {
        str.replace("INTEGER", "integer")
            .replace("INT8", "integer")
            .replace("TEXT", "text")
            .replace("VARCHAR(255)", "text")
            .replace("DATETIME", "text")
    };

    assert_eq!(
        discovered_schema
            .discover()
            .await?
            .tables
            .iter()
            .map(|table| table.write().to_string(SqliteQueryBuilder))
            .collect::<Vec<_>>(),
        vec![
            create_table.to_string(SqliteQueryBuilder),
            table_create_supplier_groups.to_string(SqliteQueryBuilder),
            table_create_suppliers.to_string(SqliteQueryBuilder),
        ]
        .into_iter()
        .map(convert_column_types)
        .collect::<Vec<_>>()
    );

    assert_eq!(
        discovered_schema
            .discover_indexes()
            .await?
            .iter()
            .map(|index| index.write().to_string(SqliteQueryBuilder))
            .collect::<Vec<_>>(),
        vec![create_index.to_string(SqliteQueryBuilder)]
    );

    Ok(())
}

async fn test_002() -> DiscoveryResult<()> {
    let connection = SqlitePool::connect("sqlite::memory:").await.unwrap();
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
        let sql = tbl_create_stmt.to_string(SqliteQueryBuilder);
        println!("{};", sql);
        println!();
        sqlx::query(&sql).execute(&mut executor).await.unwrap();
    }

    let schema_discovery = SchemaDiscovery::new(connection);

    let schema = schema_discovery.discover().await?;

    println!("{:#?}", schema);

    let map: HashMap<String, TableDef> = schema
        .tables
        .iter()
        .map(|table| (table.name.clone(), table.clone()))
        .collect();

    for tbl_create_stmt in tbl_create_stmts.into_iter() {
        let tbl_name = match tbl_create_stmt.get_table_name() {
            Some(TableRef::Table(tbl)) => tbl.to_string(),
            _ => unimplemented!(),
        };
        let expected_sql = if tbl_name.as_str() == "order" {
            vec![
                r#"CREATE TABLE "order" ("#,
                r#""id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
                r#""total" real,"#,
                r#""bakery_id" integer NOT NULL,"#,
                r#""customer_id" integer NOT NULL,"#,
                r#""placed_at" text NOT NULL,"#,
                r#"FOREIGN KEY ("customer_id") REFERENCES "customer" ("id") ON DELETE CASCADE ON UPDATE CASCADE,"#,
                r#"FOREIGN KEY ("bakery_id") REFERENCES "bakery" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
                r#")"#,
            ].join(" ")
        } else if tbl_name.as_str() == "lineitem" {
            vec![
                r#"CREATE TABLE "lineitem" ("#,
                r#""id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
                r#""price" real,"#,
                r#""quantity" integer,"#,
                r#""order_id" integer NOT NULL,"#,
                r#""cake_id" integer NOT NULL,"#,
                r#"FOREIGN KEY ("order_id") REFERENCES "order" ("id") ON DELETE CASCADE ON UPDATE CASCADE,"#,
                r#"FOREIGN KEY ("cake_id") REFERENCES "cake" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
                r#")"#,
            ].join(" ")
        } else {
            tbl_create_stmt.to_string(SqliteQueryBuilder)
        };
        let table = map.get(&tbl_name).unwrap();
        let sql = table.write().to_string(SqliteQueryBuilder);
        println!("Expected SQL:");
        println!("{};", expected_sql);
        println!("Generated SQL:");
        println!("{};", sql);
        println!();
        assert_eq!(expected_sql, sql);
    }

    Ok(())
}

fn create_bakery_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("bakery"))
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Alias::new("name")).string())
        .col(ColumnDef::new(Alias::new("profit_margin")).double())
        .to_owned()
}

fn create_baker_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("baker"))
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Alias::new("name")).string())
        .col(ColumnDef::new(Alias::new("contact_details")).json())
        .col(ColumnDef::new(Alias::new("bakery_id")).integer())
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
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Alias::new("name")).string())
        .col(ColumnDef::new(Alias::new("notes")).text())
        .to_owned()
}

fn create_order_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("order"))
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Alias::new("total")).decimal())
        .col(ColumnDef::new(Alias::new("bakery_id")).integer().not_null())
        .col(
            ColumnDef::new(Alias::new("customer_id"))
                .integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(Alias::new("placed_at"))
                .timestamp()
                .not_null(),
        )
        .foreign_key(
            ForeignKey::create()
                .from(Alias::new("order"), Alias::new("bakery_id"))
                .to(Alias::new("bakery"), Alias::new("id"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .foreign_key(
            ForeignKey::create()
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
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Alias::new("price")).decimal())
        .col(ColumnDef::new(Alias::new("quantity")).integer())
        .col(ColumnDef::new(Alias::new("order_id")).integer().not_null())
        .col(ColumnDef::new(Alias::new("cake_id")).integer().not_null())
        .foreign_key(
            ForeignKey::create()
                .from(Alias::new("lineitem"), Alias::new("cake_id"))
                .to(Alias::new("cake"), Alias::new("id"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .foreign_key(
            ForeignKey::create()
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
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Alias::new("name")).string())
        .col(ColumnDef::new(Alias::new("price")).decimal())
        .col(ColumnDef::new(Alias::new("bakery_id")).integer().not_null())
        .col(ColumnDef::new(Alias::new("gluten_free")).boolean())
        .col(ColumnDef::new(Alias::new("serial")).text())
        .foreign_key(
            ForeignKey::create()
                .from(Alias::new("cake"), Alias::new("bakery_id"))
                .to(Alias::new("bakery"), Alias::new("id"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}
