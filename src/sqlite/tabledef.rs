use crate::sqlite::{
    ColumnInfo, DefaultType, DiscoveryResult, ForeignKeysInfo, IndexInfo, IndexedColumns,
    PartialIndexInfo, PrimaryKeyAutoincrement, SqliteDiscoveryError,
};
use sea_query::{
    Alias, ColumnDef, Expr, ForeignKey, Index, Query, SqliteQueryBuilder, Table, Value,
};
use sqlx::{sqlite::SqliteRow, Row, SqliteConnection};

/// Defines a table for SQLite
#[derive(Debug, Clone)]
pub struct TableDef {
    /// The table name
    pub name: String,
    /// A list of indexes for the table
    pub indexes: Vec<IndexInfo>,
    /// A list of foreign keys in the table
    pub foreign_keys: Vec<ForeignKeysInfo>,
    /// A list of all the columns and their types
    pub columns: Vec<ColumnInfo>,
    /// Whether the primary key should autoincrement
    pub auto_increment: bool,
}

/// Gets the table name from a `SqliteRow` and maps it to the [TableDef]
impl From<&SqliteRow> for TableDef {
    fn from(row: &SqliteRow) -> Self {
        let row: String = row.get(0);
        TableDef {
            name: row,
            indexes: Vec::default(),
            foreign_keys: Vec::default(),
            columns: Vec::default(),
            auto_increment: bool::default(),
        }
    }
}

impl TableDef {
    /// Check if the primary key in the table is set to autoincrement as a result of using query
    /// `SELECT COUNT(*) from sqlite_sequence where name = 'table_name';
    pub async fn pk_is_autoincrement(
        &mut self,
        conn: &mut SqliteConnection,
    ) -> DiscoveryResult<&mut Self> {
        let check_autoincrement = Query::select()
            .expr(Expr::cust("COUNT(*)"))
            .from(Alias::new("sqlite_sequence"))
            .and_where(Expr::col(Alias::new("name")).eq(self.name.as_str()))
            .to_string(SqliteQueryBuilder);

        match sqlx::query(&check_autoincrement).fetch_one(conn).await {
            Ok(autoincrement_enabled) => {
                let autoincrement_result: &SqliteRow = &autoincrement_enabled;
                let autoincrement: PrimaryKeyAutoincrement = autoincrement_result.into();

                if autoincrement.0 == 1_u8 {
                    self.auto_increment = true;
                }

                Ok(self)
            }
            Err(err) => {
                if err.to_string().contains("no such table") {
                    Ok(self)
                } else {
                    Err(err.into())
                }
            }
        }
    }
    /// Get a list of all the indexes in the table.
    /// Note that this does not get the column name mapped by the index.
    /// To get the column name mapped by the index, the `self.get_single_indexinfo` method is invoked
    pub async fn get_indexes(&mut self, conn: &mut SqliteConnection) -> DiscoveryResult<&mut Self> {
        let mut index_query = String::default();
        index_query.push_str("pragma index_list('");
        index_query.push_str(&self.name);
        index_query.push_str("')");

        let partial_index_info_rows: Vec<SqliteRow> =
            sqlx::query(&index_query).fetch_all(&mut *conn).await?;
        let mut partial_indexes: Vec<PartialIndexInfo> = Vec::default();

        partial_index_info_rows.iter().for_each(|info| {
            let partial_index_info: PartialIndexInfo = info.into();

            if partial_index_info.origin != String::from("pk") {
                partial_indexes.push(partial_index_info);
            }
        });

        for partial_index in partial_indexes {
            let partial_index_column: IndexedColumns =
                self.get_single_indexinfo(conn, &partial_index.name).await?;

            self.indexes.push(IndexInfo {
                r#type: partial_index_column.r#type,
                index_name: partial_index_column.name,
                table_name: partial_index_column.table,
                unique: partial_index.unique,
                origin: partial_index.origin,
                partial: partial_index.partial,
                columns: partial_index_column.indexed_columns,
            });
        }

        Ok(self)
    }

    /// Get a list of all the foreign keys in the table
    pub async fn get_foreign_keys(
        &mut self,
        conn: &mut SqliteConnection,
    ) -> DiscoveryResult<&mut Self> {
        let mut index_query = String::default();
        index_query.push_str("pragma foreign_key_list('");
        index_query.push_str(&self.name);
        index_query.push_str("')");

        let index_info_rows: Vec<SqliteRow> = sqlx::query(&index_query).fetch_all(conn).await?;

        index_info_rows.iter().for_each(|info| {
            let index_info: ForeignKeysInfo = info.into();

            self.foreign_keys.push(index_info);
        });

        Ok(self)
    }

    /// Get a list of all the columns in the table mapped as [ColumnInfo]
    pub async fn get_column_info(
        &mut self,
        conn: &mut SqliteConnection,
    ) -> DiscoveryResult<&TableDef> {
        let mut index_query = String::default();
        index_query.push_str("pragma table_info('");
        index_query.push_str(&self.name);
        index_query.push_str("')");

        let index_info_rows: Vec<SqliteRow> = sqlx::query(&index_query).fetch_all(conn).await?;

        for info in index_info_rows {
            let column = ColumnInfo::to_column_def(&info)?;
            self.columns.push(column);
        }

        Ok(self)
    }

    /// Checks the column that is mapped to an index
    pub(crate) async fn get_single_indexinfo(
        &mut self,
        conn: &mut SqliteConnection,
        index_name: &str,
    ) -> DiscoveryResult<IndexedColumns> {
        let index_query = Query::select()
            .expr(Expr::cust("*"))
            .from(Alias::new("sqlite_master"))
            .and_where(Expr::col(Alias::new("name")).eq(index_name))
            .to_string(SqliteQueryBuilder);

        let index_info: &SqliteRow = &sqlx::query(&index_query).fetch_one(conn).await?;

        Ok(index_info.into())
    }

    /// Map the table definition into a SQL create statement.
    /// This is useful for simple assertions
    pub fn to_sql_statement(&self) -> String {
        let mut new_table = Table::create();
        new_table.table(Alias::new(&self.name));

        self.columns.iter().for_each(|column_info| {
            let mut new_column = ColumnDef::new(Alias::new(&column_info.name));
            if column_info.not_null {
                new_column.not_null();
            }

            if column_info.primary_key {
                new_column.primary_key();
            }

            if self.auto_increment && column_info.primary_key {
                new_column.auto_increment();
            }

            new_column.custom(Alias::new(&column_info.r#type.stringify_type()));

            match &column_info.default_value {
                DefaultType::Integer(integer_value) => {
                    new_column.default(Value::Int(Some(*integer_value)));
                }
                DefaultType::Float(float_value) => {
                    new_column.default(Value::Float(Some(*float_value)));
                }
                DefaultType::String(string_value) => {
                    new_column.default(Value::String(Some(Box::new(string_value.to_string()))));
                }
                DefaultType::Null => (),
                DefaultType::Unspecified => (),
            }

            new_table.col(&mut new_column);
        });

        self.foreign_keys.iter().for_each(|foreign_key| {
            new_table.foreign_key(
                &mut ForeignKey::create()
                    .from(Alias::new(&self.name), Alias::new(&foreign_key.from))
                    .to(Alias::new(&foreign_key.table), Alias::new(&foreign_key.to))
                    .on_delete(foreign_key.on_delete.to_seaquery_foreign_key_action())
                    .on_update(foreign_key.on_update.to_seaquery_foreign_key_action())
                    .to_owned(),
            );
        });
        let table_as_statement = new_table.to_string(SqliteQueryBuilder);

        table_as_statement
    }

    /// Maps all the information inside a table including foreign keys and indexes.
    pub fn to_sql_statement_with_indexes(&mut self) -> String {
        let mut new_table = Table::create();
        new_table.table(Alias::new(&self.name));

        self.columns.iter().for_each(|column_info| {
            let mut new_column = ColumnDef::new(Alias::new(&column_info.name));
            if column_info.not_null {
                new_column.not_null();
            }

            if column_info.primary_key {
                new_column.primary_key();
            }

            if self.auto_increment {
                new_column.auto_increment();
            }

            match &column_info.default_value {
                DefaultType::Integer(integer_value) => {
                    new_column.default(Value::Int(Some(*integer_value)));
                }
                DefaultType::Float(float_value) => {
                    new_column.default(Value::Float(Some(*float_value)));
                }
                DefaultType::String(string_value) => {
                    new_column.default(Value::String(Some(Box::new(string_value.to_string()))));
                }
                DefaultType::Null => (),
                DefaultType::Unspecified => (),
            }

            new_table.col(&mut new_column);
        });

        self.foreign_keys.iter().for_each(|foreign_key| {
            new_table.foreign_key(
                &mut ForeignKey::create()
                    .from(Alias::new(&self.name), Alias::new(&foreign_key.from))
                    .to(Alias::new(&foreign_key.table), Alias::new(&foreign_key.to))
                    .on_delete(foreign_key.on_delete.to_seaquery_foreign_key_action())
                    .on_update(foreign_key.on_update.to_seaquery_foreign_key_action())
                    .to_owned(),
            );
        });

        self.indexes.iter_mut().for_each(|index| {
            let mut new_index = Index::create();
            new_index
                .name(&index.index_name)
                .table(Alias::new(&index.table_name));

            if index.unique {
                new_index.unique();
            }

            index.columns.iter().for_each(|column| {
                new_index.col(Alias::new(&column));
            });

            new_table.index(&mut new_index);
        });

        let table_as_statement = new_table.to_string(SqliteQueryBuilder);

        table_as_statement
    }

    /// Map SQLite indexes into a sql statement
    pub fn create_indexes(&mut self) -> DiscoveryResult<Vec<String>> {
        let mut indexes: Vec<String> = Vec::default();

        if self.indexes.is_empty() {
            return Err(SqliteDiscoveryError::NoIndexesFound);
        }

        self.indexes.iter_mut().for_each(|index| {
            let mut new_index = Index::create();
            new_index
                .name(&index.index_name)
                .table(Alias::new(&index.table_name));

            if index.unique {
                new_index.unique();
            }

            index.columns.iter().for_each(|column| {
                new_index.col(Alias::new(&column));
            });

            indexes.push(new_index.to_string(SqliteQueryBuilder));
        });

        Ok(indexes)
    }
}
