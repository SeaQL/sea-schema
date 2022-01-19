use sea_orm::{Database, DbErr, TransactionTrait};
use sea_schema::migration::MigratorTrait;
use sea_schema_migration_test::Migrator;

#[async_std::main]
async fn main() -> Result<(), DbErr> {
    let url = std::env::var("DATABASE_URL").expect("Environment variable 'DATABASE_URL' not set");
    let db = &Database::connect(&url).await?;

    db.transaction(|db| {
        Box::pin(async move {
            println!("\nMigrator::install");
            Migrator::install(db).await
        })
    })
    .await
    .unwrap();

    println!("\nMigrator::up");
    Migrator::up(db, None).await?;

    println!("\nMigrator::fresh");
    Migrator::fresh(db).await?;

    println!("\nMigrator::refresh");
    Migrator::refresh(db).await?;

    println!("\nMigrator::reset");
    Migrator::reset(db).await?;

    println!("\nMigrator::status");
    Migrator::status(db).await?;

    println!("\nMigrator::down");
    Migrator::down(db, None).await?;

    Ok(())
}
