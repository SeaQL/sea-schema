use std::fmt::Display;

use super::{seaql_migrations, MigrationTrait, SchemaManager};
use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, ConnectionTrait, DbBackend, DbErr,
    EntityTrait, QueryFilter, QueryOrder, Schema, Statement,
};
use sea_query::{Alias, Expr, ForeignKey, IntoTableRef, Query, SelectStatement, Table};

#[derive(Debug, PartialEq)]
/// Status of migration
pub enum MigrationStatus {
    /// Not yet applied
    Pending,
    /// Applied
    Applied,
}

pub struct Migration {
    migration: Box<dyn MigrationTrait>,
    status: MigrationStatus,
}

impl Display for MigrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            MigrationStatus::Pending => "Pending",
            MigrationStatus::Applied => "Applied",
        };
        write!(f, "{}", status)
    }
}

/// Performing migrations on a database
#[async_trait::async_trait]
pub trait MigratorTrait {
    /// Vector of migrations in time sequence
    fn migrations() -> Vec<Box<dyn MigrationTrait>>;

    /// Get list of migrations wrapped in `Migration` struct
    fn get_migration_files() -> Vec<Migration> {
        Self::migrations()
            .into_iter()
            .map(|migration| Migration {
                migration,
                status: MigrationStatus::Pending,
            })
            .collect()
    }

    /// Get list of applied migrations from database
    async fn get_migration_models<C>(db: &C) -> Result<Vec<seaql_migrations::Model>, DbErr>
    where
        C: ConnectionTrait,
    {
        Self::install(db).await?;
        seaql_migrations::Entity::find()
            .order_by_asc(seaql_migrations::Column::Version)
            .all(db)
            .await
    }

    /// Get list of migrations with status
    async fn get_migration_with_status<C>(db: &C) -> Result<Vec<Migration>, DbErr>
    where
        C: ConnectionTrait,
    {
        Self::install(db).await?;
        let mut migration_files = Self::get_migration_files();
        let migration_models = Self::get_migration_models(db).await?;
        for (i, migration_model) in migration_models.into_iter().enumerate() {
            if let Some(migration_file) = migration_files.get_mut(i) {
                if migration_file.migration.name() == migration_model.version.as_str() {
                    migration_file.status = MigrationStatus::Applied;
                } else {
                    return Err(DbErr::Custom(format!("Migration mismatch: applied migration != migration file, '{0}' != '{1}'\nMigration '{0}' has been applied but its corresponding migration file is missing.", migration_file.migration.name(), migration_model.version)));
                }
            } else {
                return Err(DbErr::Custom(format!("Migration file of version '{}' is missing, this migration has been applied but its file is missing", migration_model.version)));
            }
        }
        Ok(migration_files)
    }

    /// Get list of pending migrations
    async fn get_pending_migrations<C>(db: &C) -> Result<Vec<Migration>, DbErr>
    where
        C: ConnectionTrait,
    {
        Self::install(db).await?;
        Ok(Self::get_migration_with_status(db)
            .await?
            .into_iter()
            .filter(|file| file.status == MigrationStatus::Pending)
            .collect())
    }

    /// Get list of applied migrations
    async fn get_applied_migrations<C>(db: &C) -> Result<Vec<Migration>, DbErr>
    where
        C: ConnectionTrait,
    {
        Self::install(db).await?;
        Ok(Self::get_migration_with_status(db)
            .await?
            .into_iter()
            .filter(|file| file.status == MigrationStatus::Applied)
            .collect())
    }

    /// Create migration table `seaql_migrations` in the database
    async fn install<C>(db: &C) -> Result<(), DbErr>
    where
        C: ConnectionTrait,
    {
        let builder = db.get_database_backend();
        let schema = Schema::new(builder);
        let mut stmt = schema.create_table_from_entity(seaql_migrations::Entity);
        stmt.if_not_exists();
        db.execute(builder.build(&stmt)).await.map(|_| ())
    }

    /// Drop all tables from the database, then reapply all migrations
    async fn fresh<C>(db: &C) -> Result<(), DbErr>
    where
        C: ConnectionTrait,
    {
        Self::install(db).await?;
        let db_backend = db.get_database_backend();

        // Temporarily disable the foreign key check
        if db_backend == DbBackend::Sqlite {
            println!("Disabling foreign key check");
            db.execute(Statement::from_string(
                db_backend,
                "PRAGMA foreign_keys = OFF".to_owned(),
            ))
            .await?;
            println!("Foreign key check disabled");
        }

        // Drop all foreign keys
        if db_backend == DbBackend::MySql {
            println!("Dropping all foreign keys");
            let mut stmt = Query::select();
            stmt.columns([Alias::new("TABLE_NAME"), Alias::new("CONSTRAINT_NAME")])
                .from((
                    Alias::new("information_schema"),
                    Alias::new("table_constraints"),
                ))
                .cond_where(
                    Condition::all()
                        .add(
                            Expr::expr(Expr::cust("DATABASE()")).equals(
                                Alias::new("table_constraints"),
                                Alias::new("table_schema"),
                            ),
                        )
                        .add(Expr::expr(Expr::value("FOREIGN KEY")).equals(
                            Alias::new("table_constraints"),
                            Alias::new("constraint_type"),
                        )),
                );
            let rows = db.query_all(db_backend.build(&stmt)).await?;
            for row in rows.into_iter() {
                let constraint_name: String = row.try_get("", "CONSTRAINT_NAME")?;
                let table_name: String = row.try_get("", "TABLE_NAME")?;
                println!(
                    "Dropping foreign key '{}' from table '{}'",
                    constraint_name, table_name
                );
                let mut stmt = ForeignKey::drop();
                stmt.table(Alias::new(table_name.as_str()))
                    .name(constraint_name.as_str());
                db.execute(db_backend.build(&stmt)).await?;
                println!("Foreign key '{}' has been dropped", constraint_name);
            }
            println!("All foreign keys dropped");
        }

        // Drop all tables
        let stmt = query_tables(db);
        let rows = db.query_all(db_backend.build(&stmt)).await?;
        for row in rows.into_iter() {
            let table_name: String = row.try_get("", "table_name")?;
            println!("Dropping table '{}'", table_name);
            let mut stmt = Table::drop();
            stmt.table(Alias::new(table_name.as_str()))
                .if_exists()
                .cascade();
            db.execute(db_backend.build(&stmt)).await?;
            println!("Table '{}' has been dropped", table_name);
        }

        // Restore the foreign key check
        if db_backend == DbBackend::Sqlite {
            println!("Restoring foreign key check");
            db.execute(Statement::from_string(
                db_backend,
                "PRAGMA foreign_keys = ON".to_owned(),
            ))
            .await?;
            println!("Foreign key check restored");
        }

        // Reapply all migrations
        Self::up(db, None).await
    }

    /// Rollback all applied migrations, then reapply all migrations
    async fn refresh<C>(db: &C) -> Result<(), DbErr>
    where
        C: ConnectionTrait,
    {
        Self::down(db, None).await?;
        Self::up(db, None).await
    }

    /// Rollback all applied migrations
    async fn reset<C>(db: &C) -> Result<(), DbErr>
    where
        C: ConnectionTrait,
    {
        Self::down(db, None).await
    }

    /// Check the status of all migrations
    async fn status<C>(db: &C) -> Result<(), DbErr>
    where
        C: ConnectionTrait,
    {
        Self::install(db).await?;

        for Migration { migration, status } in Self::get_migration_with_status(db).await? {
            println!("Migration '{}'... {}", migration.name(), status);
        }

        Ok(())
    }

    /// Apply pending migrations
    async fn up<C>(db: &C, mut steps: Option<u32>) -> Result<(), DbErr>
    where
        C: ConnectionTrait,
    {
        Self::install(db).await?;
        let manager = SchemaManager::new(db);

        for Migration { migration, .. } in Self::get_pending_migrations(db).await?.into_iter() {
            if let Some(steps) = steps.as_mut() {
                if steps == &0 {
                    break;
                }
                *steps -= 1;
            }
            println!("Appling migration '{}'", migration.name());
            migration.up(&manager).await?;
            println!("Migration '{}' has been applied", migration.name());
            seaql_migrations::ActiveModel {
                version: ActiveValue::Set(migration.name().to_owned()),
                applied_at: ActiveValue::Set(Local::now().into()),
            }
            .insert(db)
            .await?;
        }

        Ok(())
    }

    /// Rollback applied migrations
    async fn down<C>(db: &C, mut steps: Option<u32>) -> Result<(), DbErr>
    where
        C: ConnectionTrait,
    {
        Self::install(db).await?;
        let manager = SchemaManager::new(db);

        for Migration { migration, .. } in Self::get_applied_migrations(db).await?.into_iter().rev()
        {
            if let Some(steps) = steps.as_mut() {
                if steps == &0 {
                    break;
                }
                *steps -= 1;
            }
            println!("Rolling back migration '{}'", migration.name());
            migration.down(&manager).await?;
            println!("Migration '{}' has been rollbacked", migration.name());
            seaql_migrations::Entity::delete_many()
                .filter(seaql_migrations::Column::Version.eq(migration.name()))
                .exec(db)
                .await?;
        }

        Ok(())
    }
}

pub(crate) fn query_tables<C>(db: &C) -> SelectStatement
where
    C: ConnectionTrait + ?Sized,
{
    let mut stmt = Query::select();
    let (expr, tbl_ref, condition) = match db.get_database_backend() {
        DbBackend::MySql => (
            Expr::col(Alias::new("table_name")),
            (Alias::new("information_schema"), Alias::new("tables")).into_table_ref(),
            Condition::all().add(
                Expr::expr(Expr::cust("DATABASE()"))
                    .equals(Alias::new("tables"), Alias::new("table_schema")),
            ),
        ),
        DbBackend::Postgres => (
            Expr::col(Alias::new("table_name")),
            (Alias::new("information_schema"), Alias::new("tables")).into_table_ref(),
            Condition::all()
                .add(
                    Expr::expr(Expr::cust("CURRENT_SCHEMA()"))
                        .equals(Alias::new("tables"), Alias::new("table_schema")),
                )
                .add(Expr::col(Alias::new("table_type")).eq("BASE TABLE")),
        ),
        DbBackend::Sqlite => (
            Expr::col(Alias::new("name")),
            Alias::new("sqlite_master").into_table_ref(),
            Condition::all()
                .add(Expr::col(Alias::new("type")).eq("table"))
                .add(Expr::col(Alias::new("name")).ne("sqlite_sequence")),
        ),
    };
    stmt.expr_as(expr, Alias::new("table_name"))
        .from(tbl_ref)
        .cond_where(condition);
    stmt
}
