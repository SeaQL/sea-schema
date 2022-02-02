use sea_schema::migration::*;
use sea_schema_migration_test::Migrator;

#[async_std::main]
async fn main() {
    cli::run_cli(Migrator).await;
}
