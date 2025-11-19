use pretty_assertions::assert_eq;
use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;
use std::collections::HashMap;

use sea_schema::sea_query::{
    ColumnDef, Expr, ForeignKey, ForeignKeyAction, ForeignKeyCreateStatement, Index, Query,
    SqliteQueryBuilder, Table, TableCreateStatement, TableName, TableRef,
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
    let url = std::env::var("DATABASE_URL_LIVE").unwrap_or_else(|_| "sqlite::memory:".to_owned());

    let sqlite_pool = SqlitePoolOptions::new().connect(&url).await.unwrap();

    //DROP TABLES to ensure all tests pass
    sqlx::query("DROP TABLE IF EXISTS Programming_Langs")
        .fetch_all(&mut *sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query("DROP TABLE IF EXISTS suppliers")
        .fetch_all(&mut *sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query("DROP TABLE IF EXISTS supplier_groups")
        .fetch_all(&mut *sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    // Creates a new test table
    let create_table = Table::create()
        .table("Programming_Langs")
        .col(
            ColumnDef::new("Name")
                .big_integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new("SLOC")
                .big_integer()
                .not_null()
                .default(100.45),
        )
        .col(ColumnDef::new("SemVer").string_len(255).not_null())
        .col(
            ColumnDef::new("last_update")
                .custom("DATETIME")
                .not_null()
                .default("1990-01-01 00:00:00"),
        )
        .index(Index::create().col("SLOC").col("SemVer").unique())
        .to_owned();

    // This ensures that the `sqlite_sequence` table is populated

    let insert_into_table = Query::insert()
        .into_table("Programming_Langs")
        .columns(["SLOC", "SemVer"])
        .values([4.into(), "0.1.0".into()])
        .unwrap()
        .to_owned();

    let create_index = Index::create()
        .name("idx-programming_lang-aspect")
        .table("Programming_Langs")
        .col("SLOC")
        .col("SemVer")
        .to_owned();

    // Create a table with a PRIMARY KEY constraint that results in an index.
    let create_table_inventors = Table::create()
        .table("Inventors")
        .col(ColumnDef::new("Name").text().not_null())
        .col(ColumnDef::new("Invention").string().not_null())
        .index(Index::create().col("Invention").unique())
        .index(Index::create().col("Name").primary())
        .to_owned();

    //DROP TABLES to ensure all tests pass
    sqlx::query("DROP TABLE IF EXISTS Programming_Langs")
        .fetch_all(&mut *sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query("DROP TABLE IF EXISTS suppliers")
        .fetch_all(&mut *sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query("DROP TABLE IF EXISTS supplier_groups")
        .fetch_all(&mut *sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    // Tests foreign key discovery
    let table_create_suppliers = Table::create()
        .table("suppliers")
        .col(ColumnDef::new("supplier_id").integer().not_null())
        .col(ColumnDef::new("supplier_name").text().not_null())
        .col(ColumnDef::new("group_id").integer().not_null())
        .primary_key(Index::create().col("supplier_id"))
        .foreign_key(
            ForeignKeyCreateStatement::new()
                .name("group_id")
                .from("suppliers", "group_id")
                .to("supplier_groups", "group_id")
                .on_delete(ForeignKeyAction::SetNull)
                .on_update(ForeignKeyAction::SetNull),
        )
        .to_owned();

    let table_create_supplier_groups = Table::create()
        .table("supplier_groups")
        .col(ColumnDef::new("group_id").integer().not_null())
        .col(ColumnDef::new("group_name").text().not_null())
        .primary_key(Index::create().col("group_id"))
        .to_owned();

    let table_create_supplier_groups_expected = Table::create()
        .table("supplier_groups")
        .col(ColumnDef::new("group_id").integer().not_null())
        .col(ColumnDef::new("group_name").text().not_null())
        .primary_key(Index::create().col("group_id"))
        .to_owned();

    let insert_into_supplier_groups = Query::insert()
        .into_table("supplier_groups")
        .columns(["group_name"])
        .values(["Global".into()])
        .unwrap()
        .to_owned();

    let insert_into_suppliers = Query::insert()
        .into_table("suppliers")
        .columns(["supplier_name", "group_id"])
        .values(["HP".into(), 1.into()])
        .unwrap()
        .to_owned();

    sqlx::query(&create_table.to_string(SqliteQueryBuilder))
        .fetch_all(&mut *sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query(&create_table_inventors.to_string(SqliteQueryBuilder))
        .fetch_all(&mut *sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query(&insert_into_table.to_string(SqliteQueryBuilder))
        .fetch_all(&mut *sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query(&table_create_supplier_groups.to_string(SqliteQueryBuilder))
        .fetch_all(&mut *sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query(&table_create_suppliers.to_string(SqliteQueryBuilder))
        .fetch_all(&mut *sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query(&insert_into_supplier_groups.to_string(SqliteQueryBuilder))
        .fetch_all(&mut *sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query(&insert_into_suppliers.to_string(SqliteQueryBuilder))
        .fetch_all(&mut *sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    sqlx::query(&create_index.to_string(SqliteQueryBuilder))
        .fetch_all(&mut *sqlite_pool.acquire().await.unwrap())
        .await
        .unwrap();

    let schema = SchemaDiscovery::new(sqlite_pool.clone()).discover().await?;

    let expected_sql = [
        create_table.to_string(SqliteQueryBuilder),
        create_table_inventors.to_string(SqliteQueryBuilder),
        table_create_supplier_groups_expected.to_string(SqliteQueryBuilder),
        table_create_suppliers.to_string(SqliteQueryBuilder),
    ]
    .into_iter()
    .collect::<Vec<_>>();
    assert_eq!(schema.tables.len(), expected_sql.len());

    for (i, table) in schema.tables.into_iter().enumerate() {
        let sql = table.write().to_string(SqliteQueryBuilder);
        if sql == expected_sql[i] {
            println!("[OK] {sql}");
        }
        assert_eq!(sql, expected_sql[i]);
    }

    let expected_sql = [create_index.to_string(SqliteQueryBuilder)]
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(schema.indexes.len(), expected_sql.len());

    for (i, index) in schema.indexes.into_iter().enumerate() {
        let sql = index.write().to_string(SqliteQueryBuilder);
        if sql == expected_sql[i] {
            println!("[OK] {sql}");
        }
        assert_eq!(sql, expected_sql[i]);
    }

    Ok(())
}

async fn test_002() -> DiscoveryResult<()> {
    let url = std::env::var("DATABASE_URL_LIVE").unwrap_or_else(|_| "sqlite::memory:".to_owned());

    let connection = SqlitePool::connect(&url).await.unwrap();
    let mut executor = connection.acquire().await.unwrap();

    let tbl_create_stmts = vec![
        create_bakery_table(),
        create_baker_table(),
        create_customer_table(),
        create_order_table(),
        create_cake_table(),
        create_cakes_bakers_table(),
        create_lineitem_table(),
        create_parent_table(),
        create_child_table(),
        create_strange_table(),
    ];

    for tbl_create_stmt in tbl_create_stmts.iter() {
        let sql = tbl_create_stmt.to_string(SqliteQueryBuilder);
        sqlx::query(&sql).execute(&mut *executor).await.unwrap();
    }

    let schema_discovery = SchemaDiscovery::new(connection);

    let schema = schema_discovery.discover().await?;

    // println!("{:#?}", schema);

    let map: HashMap<String, TableDef> = schema
        .tables
        .iter()
        .map(|table| (table.name.clone(), table.clone()))
        .collect();

    for tbl_create_stmt in tbl_create_stmts.into_iter() {
        let tbl_name = match tbl_create_stmt.get_table_name() {
            Some(TableRef::Table(TableName(_, tbl), _)) => tbl.to_string(),
            _ => unimplemented!(),
        };
        let expected_sql = if tbl_name.as_str() == "order" {
            [
                r#"CREATE TABLE "order" ("#,
                r#""id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
                r#""total" decimal,"#,
                r#""bakery_id" integer NOT NULL,"#,
                r#""customer_id" integer NOT NULL,"#,
                r#""placed_at" datetime_text NOT NULL DEFAULT CURRENT_TIMESTAMP,"#,
                r#""updated" datetime_text NOT NULL DEFAULT '2023-06-07 16:24:00',"#,
                r#""net_weight" double NOT NULL DEFAULT 10.05,"#,
                r#""priority" integer NOT NULL DEFAULT 5,"#,
                r#"FOREIGN KEY ("customer_id") REFERENCES "customer" ("id") ON DELETE CASCADE ON UPDATE CASCADE,"#,
                r#"FOREIGN KEY ("bakery_id") REFERENCES "bakery" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
                r#")"#,
            ].join(" ")
        } else if tbl_name.as_str() == "lineitem" {
            [
                r#"CREATE TABLE "lineitem" ("#,
                r#""id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
                r#""price" decimal,"#,
                r#""quantity" integer,"#,
                r#""order_id" integer NOT NULL,"#,
                r#""cake_id" integer NOT NULL,"#,
                r#"UNIQUE ("cake_id", "order_id"),"#,
                r#"UNIQUE ("cake_id"),"#,
                r#"FOREIGN KEY ("order_id") REFERENCES "order" ("id") ON DELETE CASCADE ON UPDATE CASCADE,"#,
                r#"FOREIGN KEY ("cake_id") REFERENCES "cake" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
                r#")"#,
            ].join(" ")
        } else {
            tbl_create_stmt.to_string(SqliteQueryBuilder)
        };
        let table = map.get(&tbl_name).unwrap();
        let sql = table.write().to_string(SqliteQueryBuilder);
        if expected_sql == sql {
            println!("[OK] {sql}");
        }
        assert_eq!(expected_sql, sql);
    }

    Ok(())
}

fn create_bakery_table() -> TableCreateStatement {
    Table::create()
        .table("bakery")
        .col(
            ColumnDef::new("id")
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new("name").string())
        .col(ColumnDef::new("profit_margin").double())
        .to_owned()
}

fn create_baker_table() -> TableCreateStatement {
    Table::create()
        .table("baker")
        .col(
            ColumnDef::new("id")
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new("name").string())
        .col(ColumnDef::new("contact_details").json())
        .col(ColumnDef::new("bakery_id").integer())
        .foreign_key(
            ForeignKey::create()
                .name("FK_baker_bakery")
                .from("baker", "bakery_id")
                .to("bakery", "id")
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_customer_table() -> TableCreateStatement {
    Table::create()
        .table("customer")
        .col(
            ColumnDef::new("id")
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new("name").string())
        .col(ColumnDef::new("notes").text())
        .to_owned()
}

fn create_order_table() -> TableCreateStatement {
    Table::create()
        .table("order")
        .col(
            ColumnDef::new("id")
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new("total").decimal())
        .col(ColumnDef::new("bakery_id").integer().not_null())
        .col(ColumnDef::new("customer_id").integer().not_null())
        .col(
            ColumnDef::new("placed_at")
                .date_time()
                .not_null()
                .default(Expr::current_timestamp()),
        )
        .col(
            ColumnDef::new("updated")
                .date_time()
                .not_null()
                .default("2023-06-07 16:24:00"),
        )
        .col(
            ColumnDef::new("net_weight")
                .double()
                .not_null()
                .default(10.05),
        )
        .col(ColumnDef::new("priority").integer().not_null().default(5))
        .foreign_key(
            ForeignKey::create()
                .from("order", "bakery_id")
                .to("bakery", "id")
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .foreign_key(
            ForeignKey::create()
                .from("order", "customer_id")
                .to("customer", "id")
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_lineitem_table() -> TableCreateStatement {
    Table::create()
        .table("lineitem")
        .col(
            ColumnDef::new("id")
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new("price").decimal())
        .col(ColumnDef::new("quantity").integer())
        .col(ColumnDef::new("order_id").integer().not_null())
        .col(ColumnDef::new("cake_id").integer().not_null())
        .index(
            Index::create()
                .unique()
                .name("UNI_lineitem_cake_id")
                .col("cake_id"),
        )
        .index(
            Index::create()
                .unique()
                .name("UNI_lineitem_cake_id_order_id")
                .col("cake_id")
                .col("order_id"),
        )
        .foreign_key(
            ForeignKey::create()
                .from("lineitem", "cake_id")
                .to("cake", "id")
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .foreign_key(
            ForeignKey::create()
                .from("lineitem", "order_id")
                .to("order", "id")
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_cakes_bakers_table() -> TableCreateStatement {
    Table::create()
        .table("cakes_bakers")
        .col(ColumnDef::new("cake_id").integer().not_null())
        .col(ColumnDef::new("baker_id").integer().not_null())
        .primary_key(Index::create().col("cake_id").col("baker_id"))
        .to_owned()
}

fn create_cake_table() -> TableCreateStatement {
    Table::create()
        .table("cake")
        .col(
            ColumnDef::new("id")
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new("name").string())
        .col(ColumnDef::new("price").decimal())
        .col(ColumnDef::new("bakery_id").integer().not_null())
        .col(ColumnDef::new("gluten_free").boolean())
        .col(ColumnDef::new("serial").text())
        .foreign_key(
            ForeignKey::create()
                .from("cake", "bakery_id")
                .to("bakery", "id")
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_parent_table() -> TableCreateStatement {
    Table::create()
        .table("parent")
        .col(ColumnDef::new("id1").integer().not_null())
        .col(ColumnDef::new("id2").integer().not_null())
        .primary_key(Index::create().primary().col("id1").col("id2"))
        .to_owned()
}

fn create_child_table() -> TableCreateStatement {
    Table::create()
        .table("child")
        .col(
            ColumnDef::new("id")
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new("parent_id1").integer().not_null())
        .col(ColumnDef::new("parent_id2").integer().not_null())
        .foreign_key(
            ForeignKey::create()
                .name("FK_child_parent")
                .from("child", ("parent_id1", "parent_id2"))
                .to("parent", ("id1", "id2"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_strange_table() -> TableCreateStatement {
    Table::create()
        .table("strange")
        .col(
            ColumnDef::new("id")
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new("int1").integer())
        .col(ColumnDef::new("int2").tiny_integer())
        .col(ColumnDef::new("int3").small_integer())
        .col(ColumnDef::new("int4").big_integer())
        .col(ColumnDef::new("string1").string())
        .col(ColumnDef::new("string2").string_len(24))
        .col(ColumnDef::new("char1").char())
        .col(ColumnDef::new("char2").char_len(24))
        .col(ColumnDef::new("text_col").text())
        .col(ColumnDef::new("json_col").json())
        .col(ColumnDef::new("uuid_col").uuid())
        .col(ColumnDef::new("decimal1").decimal())
        .col(ColumnDef::new("decimal2").decimal_len(12, 4))
        .col(ColumnDef::new("money1").money())
        .col(ColumnDef::new("money2").money_len(12, 4))
        .col(ColumnDef::new("float_col").float())
        .col(ColumnDef::new("double_col").double())
        .col(ColumnDef::new("date_col").date())
        .col(ColumnDef::new("time_col").time())
        .col(ColumnDef::new("datetime_col").date_time())
        .col(ColumnDef::new("boolean_col").boolean())
        .col(ColumnDef::new("binary2").binary_len(1024))
        .col(ColumnDef::new("binary3").var_binary(1024))
        .to_owned()
}
