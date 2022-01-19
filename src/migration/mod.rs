pub mod manager;
pub mod migrator;
pub mod seaql_migrations;

pub use manager::*;
pub use migrator::*;
pub use seaql_migrations::*;

pub use sea_orm::DbErr;

pub trait MigrationName {
    fn name(&self) -> &str;
}

/// The migration definition
#[async_trait::async_trait]
pub trait MigrationTrait: MigrationName + Send + Sync {
    /// Define actions to perform when applying the migration
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>;

    /// Define actions to perform when rolling back the migration
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>;
}
