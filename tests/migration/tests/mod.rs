use sea_orm::{Database, DbErr};
use sea_schema::migration::*;
use sea_schema_migration_test::Migrator;

#[async_std::test]
async fn main() -> Result<(), DbErr> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .init();

    let url = std::env::var("DATABASE_URL").expect("Environment variable 'DATABASE_URL' not set");
    let db = &Database::connect(&url).await?;
    let manager = SchemaManager::new(db);

    println!("\nMigrator::status");
    Migrator::status(db).await?;

    println!("\nMigrator::install");
    Migrator::install(db).await?;

    assert!(manager.has_table("seaql_migrations").await?);

    println!("\nMigrator::reset");
    Migrator::reset(db).await?;

    assert!(!manager.has_table("cake").await?);
    assert!(!manager.has_table("fruit").await?);

    println!("\nMigrator::up");
    Migrator::up(db, Some(0)).await?;

    assert!(!manager.has_table("cake").await?);
    assert!(!manager.has_table("fruit").await?);

    println!("\nMigrator::up");
    Migrator::up(db, Some(1)).await?;

    assert!(manager.has_table("cake").await?);
    assert!(!manager.has_table("fruit").await?);

    println!("\nMigrator::down");
    Migrator::down(db, Some(0)).await?;

    assert!(manager.has_table("cake").await?);
    assert!(!manager.has_table("fruit").await?);

    println!("\nMigrator::down");
    Migrator::down(db, Some(1)).await?;

    assert!(!manager.has_table("cake").await?);
    assert!(!manager.has_table("fruit").await?);

    println!("\nMigrator::up");
    Migrator::up(db, None).await?;

    println!("\nMigrator::status");
    Migrator::status(db).await?;

    assert!(manager.has_table("cake").await?);
    assert!(manager.has_table("fruit").await?);

    println!("\nMigrator::down");
    Migrator::down(db, None).await?;

    assert!(manager.has_table("seaql_migrations").await?);
    assert!(!manager.has_table("cake").await?);
    assert!(!manager.has_table("fruit").await?);

    println!("\nMigrator::fresh");
    Migrator::fresh(db).await?;

    assert!(manager.has_table("cake").await?);
    assert!(manager.has_table("fruit").await?);

    println!("\nMigrator::refresh");
    Migrator::refresh(db).await?;

    assert!(manager.has_table("cake").await?);
    assert!(manager.has_table("fruit").await?);

    println!("\nMigrator::reset");
    Migrator::reset(db).await?;

    assert!(!manager.has_table("cake").await?);
    assert!(!manager.has_table("fruit").await?);

    println!("\nMigrator::status");
    Migrator::status(db).await?;

    Ok(())
}
