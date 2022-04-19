use super::{query_tables, ConnectionTrait, DbBackend, MigrationErr, Statement, StatementBuilder};
use sea_query::{
    extension::postgres::{TypeAlterStatement, TypeCreateStatement, TypeDropStatement},
    Alias, Condition, Expr, ForeignKeyCreateStatement, ForeignKeyDropStatement,
    IndexCreateStatement, IndexDropStatement, Query, TableAlterStatement, TableCreateStatement,
    TableDropStatement, TableRenameStatement, TableTruncateStatement,
};

/// Helper struct for writing migration scripts in migration file
pub struct SchemaManager;

/// Schema Creation
impl SchemaManager {
    /// Create table
    pub async fn create_table(
        stmt: TableCreateStatement,
        conn: &dyn ConnectionTrait,
    ) -> Result<(), MigrationErr> {
        execute(stmt, conn).await
    }

    /// Create index
    pub async fn create_index(
        stmt: IndexCreateStatement,
        conn: &dyn ConnectionTrait,
    ) -> Result<(), MigrationErr> {
        execute(stmt, conn).await
    }

    /// Create foreign key
    pub async fn create_foreign_key(
        stmt: ForeignKeyCreateStatement,
        conn: &dyn ConnectionTrait,
    ) -> Result<(), MigrationErr> {
        execute(stmt, conn).await
    }

    /// Create type
    pub async fn create_type(
        stmt: TypeCreateStatement,
        conn: &dyn ConnectionTrait,
    ) -> Result<(), MigrationErr> {
        execute(stmt, conn).await
    }
}

/// Schema Mutation
impl SchemaManager {
    /// Alter table
    pub async fn alter_table(
        stmt: TableAlterStatement,
        conn: &dyn ConnectionTrait,
    ) -> Result<(), MigrationErr> {
        execute(stmt, conn).await
    }

    /// Drop table
    pub async fn drop_table(
        stmt: TableDropStatement,
        conn: &dyn ConnectionTrait,
    ) -> Result<(), MigrationErr> {
        execute(stmt, conn).await
    }

    /// Rename table
    pub async fn rename_table(
        stmt: TableRenameStatement,
        conn: &dyn ConnectionTrait,
    ) -> Result<(), MigrationErr> {
        execute(stmt, conn).await
    }

    /// Truncate table
    pub async fn truncate_table(
        stmt: TableTruncateStatement,
        conn: &dyn ConnectionTrait,
    ) -> Result<(), MigrationErr> {
        execute(stmt, conn).await
    }

    /// Drop index
    pub async fn drop_index(
        stmt: IndexDropStatement,
        conn: &dyn ConnectionTrait,
    ) -> Result<(), MigrationErr> {
        execute(stmt, conn).await
    }

    /// Drop foreign key
    pub async fn drop_foreign_key(
        stmt: ForeignKeyDropStatement,
        conn: &dyn ConnectionTrait,
    ) -> Result<(), MigrationErr> {
        execute(stmt, conn).await
    }

    /// Alter type
    pub async fn alter_type(
        stmt: TypeAlterStatement,
        conn: &dyn ConnectionTrait,
    ) -> Result<(), MigrationErr> {
        execute(stmt, conn).await
    }

    /// Drop type
    pub async fn drop_type(
        stmt: TypeDropStatement,
        conn: &dyn ConnectionTrait,
    ) -> Result<(), MigrationErr> {
        execute(stmt, conn).await
    }
}

/// Schema Inspection
impl SchemaManager {
    /// Check if a table exists in the database
    pub async fn has_table<T>(table: T, conn: &dyn ConnectionTrait) -> Result<bool, MigrationErr>
    where
        T: AsRef<str>,
    {
        let mut stmt = Query::select();
        let mut subquery = query_tables(conn);
        subquery.cond_where(Expr::col(Alias::new("table_name")).eq(table.as_ref()));
        stmt.expr_as(Expr::cust("COUNT(*)"), Alias::new("rows"))
            .from_subquery(subquery, Alias::new("subquery"));

        let builder = conn.get_database_backend();
        let res = conn
            .query_one(builder.build(&stmt))
            .await?
            .ok_or_else(|| MigrationErr("Fail to check table exists".to_owned()))?;
        let rows = res.try_get_i64("rows")?;

        Ok(rows > 0)
    }

    /// Check if a column exists in a specific database table
    pub async fn has_column<T, C>(
        table: T,
        column: C,
        conn: &dyn ConnectionTrait,
    ) -> Result<bool, MigrationErr>
    where
        T: AsRef<str>,
        C: AsRef<str>,
    {
        let db_backend = conn.get_database_backend();
        let found = match db_backend {
            DbBackend::MySql | DbBackend::Postgres => {
                let schema_name = match db_backend {
                    DbBackend::MySql => "DATABASE()",
                    DbBackend::Postgres => "CURRENT_SCHEMA()",
                    DbBackend::Sqlite => unreachable!(),
                };
                let mut stmt = Query::select();
                stmt.expr_as(Expr::cust("COUNT(*)"), Alias::new("rows"))
                    .from((Alias::new("information_schema"), Alias::new("columns")))
                    .cond_where(
                        Condition::all()
                            .add(
                                Expr::expr(Expr::cust(schema_name))
                                    .equals(Alias::new("columns"), Alias::new("table_schema")),
                            )
                            .add(Expr::col(Alias::new("table_name")).eq(table.as_ref()))
                            .add(Expr::col(Alias::new("column_name")).eq(column.as_ref())),
                    );

                let res = conn
                    .query_one(db_backend.build(&stmt))
                    .await?
                    .ok_or_else(|| MigrationErr("Fail to check column exists".to_owned()))?;
                let rows = res.try_get_i64("rows")?;
                rows > 0
            }
            DbBackend::Sqlite => {
                let stmt = Statement::from_string(
                    db_backend,
                    format!("PRAGMA table_info({})", table.as_ref()),
                );
                let results = conn.query_all(stmt).await?;
                let mut found = false;
                for res in results {
                    let name = res.try_get_string("name")?;
                    if name.as_str() == column.as_ref() {
                        found = true;
                    }
                }
                found
            }
        };
        Ok(found)
    }
}

async fn execute<S>(stmt: S, conn: &dyn ConnectionTrait) -> Result<(), MigrationErr>
where
    S: StatementBuilder,
{
    let builder = conn.get_database_backend();
    conn.execute(builder.build(&stmt)).await
}
