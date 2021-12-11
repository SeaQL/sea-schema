use async_std::fs::File;
use sea_query::{
    Alias, ColumnDef, ForeignKeyAction, ForeignKeyCreateStatement, Index, Query,
    SqliteQueryBuilder, Table,
};
use sea_schema::sqlite::SchemaDiscovery;
use sqlx::sqlite::SqlitePoolOptions;

#[cfg_attr(test, async_std::test)]
#[cfg_attr(not(test), async_std::main)]
async fn main() {
    File::create("test.db").await.unwrap();
    let sqlite_pool = SqlitePoolOptions::new()
        .connect("sqlite://test.db")
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
                .default("CURRENT_TIMESTAMP"),
        )
        .to_owned();

    // This ensures that the `sqlite_sequence` table is populated

    let insert_into_table = Query::insert()
        .into_table(Alias::new("Programming_Langs"))
        .columns(vec![Alias::new("SLOC"), Alias::new("SemVer")])
        .values(vec![4.into(), "0.1.0".into()])
        .unwrap()
        .to_owned();

    //dbg!(&create_table.to_string(SqliteQueryBuilder));

    let create_index = Index::create()
        .name("idx-programming_lang-aspect")
        .table(Alias::new("Programming_Langs"))
        .col(Alias::new("SLOC"))
        .col(Alias::new("SemVer"))
        .to_owned();

    dbg!(&create_index.to_string(SqliteQueryBuilder));

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
        .col(
            ColumnDef::new(Alias::new("supplier_id"))
                .custom(Alias::new("INTEGER"))
                .primary_key(),
        )
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
        .col(
            ColumnDef::new(Alias::new("group_id"))
                .custom(Alias::new("INTEGER"))
                .primary_key(),
        )
        .col(
            ColumnDef::new(Alias::new("group_name"))
                .custom(Alias::new("TEXT"))
                .not_null(),
        )
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

    dbg!(&insert_into_supplier_groups.to_string(SqliteQueryBuilder));

    let insert_into_suppliers = Query::insert()
        .into_table(Alias::new("suppliers"))
        .columns(vec![Alias::new("supplier_name"), Alias::new("group_id")])
        .values(vec!["HP".into(), 1.into()])
        .unwrap()
        .to_owned();

    dbg!(&insert_into_suppliers.to_string(SqliteQueryBuilder));

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

    let mut discovered_schema = SchemaDiscovery::new(sqlite_pool.clone());
    let discover_tables = discovered_schema.discover().await.unwrap();

    // Doing a binary search instead of an assertion on Vec indexes since they can panic
    // due to re-arrangement of indexes between queries

    let discovered_schema_statements = discover_tables
        .tables
        .iter()
        .map(|table| table.write().to_string(SqliteQueryBuilder))
        .collect::<Vec<String>>();

    dbg!(&discovered_schema_statements.binary_search(&create_table.to_string(SqliteQueryBuilder)));
    dbg!(&discovered_schema_statements
        .binary_search(&table_create_suppliers.to_string(SqliteQueryBuilder)));
    dbg!(&discovered_schema_statements
        .binary_search(&table_create_supplier_groups.to_string(SqliteQueryBuilder)));
    assert!(match &discovered_schema_statements
        .binary_search(&create_table.to_string(SqliteQueryBuilder))
    {
        Ok(_) => true,
        Err(_) => false,
    });
    assert!(match &discovered_schema_statements
        .binary_search(&table_create_suppliers.to_string(SqliteQueryBuilder))
    {
        Ok(_) => true,
        Err(_) => false,
    });
    assert!(match &discovered_schema_statements
        .binary_search(&table_create_supplier_groups.to_string(SqliteQueryBuilder))
    {
        Ok(_) => true,
        Err(_) => false,
    });

    let discover_indexes = SchemaDiscovery::new(sqlite_pool)
        .discover_indexes()
        .await
        .unwrap();

    dbg!(&discover_indexes);

    let mut index_create_statements = Vec::default();
    discover_indexes.iter().for_each(|index| {
        let index_statement: sea_query::IndexCreateStatement = index.write();
        index_create_statements.push(index_statement.to_string(SqliteQueryBuilder));
    });

    assert!(
        match index_create_statements.binary_search(&create_index.to_string(SqliteQueryBuilder)) {
            Ok(_) => true,
            Err(_) => false,
        }
    );
}

/*
use sea_query::{
    Alias, ColumnDef, ForeignKeyAction, ForeignKeyCreateStatement, Query, SqliteQueryBuilder, Table,
};
use sea_schema::sqlite::SchemaDiscovery;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

#[cfg_attr(test, async_std::test)]
#[cfg_attr(not(test), async_std::main)]
async fn main() {

}
*/
