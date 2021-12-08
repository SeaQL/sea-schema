use sea_query::{Alias, ColumnDef, Query, SqliteQueryBuilder, Table};
use sea_schema::sqlite::SchemaDiscovery;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    ConnectOptions,
};
use std::str::FromStr;

#[async_std::main]
async fn main() {
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
                .not_null(),
        )
        .col(
            ColumnDef::new(Alias::new("SemVer"))
                .custom(Alias::new("VARCHAR(255)"))
                .not_null(),
        )
        .to_owned();

    // This ensures that the `sqlite_sequence` table is populated

    let insert_into_table = Query::insert()
        .into_table(Alias::new("Programming_Langs"))
        .columns(vec![Alias::new("SLOC"), Alias::new("SemVer")])
        .values(vec![4.into(), "0.1.0".into()])
        .unwrap()
        .to_owned();

    dbg!(&create_table.to_string(SqliteQueryBuilder));

    let mut sqlite_connection = SqliteConnectOptions::from_str("sqlite://foo.db")
        .unwrap()
        .journal_mode(SqliteJournalMode::Wal)
        .connect()
        .await
        .unwrap();

    sqlx::query(&create_table.to_string(SqliteQueryBuilder))
        .fetch_all(&mut sqlite_connection)
        .await
        .unwrap();

    sqlx::query(&insert_into_table.to_string(SqliteQueryBuilder))
        .fetch_all(&mut sqlite_connection)
        .await
        .unwrap();

    assert_eq!(
        create_table.to_string(SqliteQueryBuilder),
        SchemaDiscovery::new("foo.db")
            .await
            .unwrap()
            .discover()
            .await
            .unwrap()
            .to_sql()[0]
    );
}
