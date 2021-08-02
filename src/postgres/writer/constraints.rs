use crate::postgres::def::{PrimaryKey, References, Unique};
use sea_query::{Alias, ForeignKey, ForeignKeyCreateStatement, Index, IndexCreateStatement};

impl PrimaryKey {
    pub fn write(&self) -> IndexCreateStatement {
        let mut idx = Index::create().primary();
        for col in self.0.iter() {
            idx = idx.col(Alias::new(col));
        }
        idx
    }
}

impl Unique {
    pub fn write(&self) -> IndexCreateStatement {
        let mut idx = Index::create().unique();
        for col in self.0.iter() {
            idx = idx.col(Alias::new(col));
        }
        idx
    }
}

impl References {
    pub fn write(&self) -> ForeignKeyCreateStatement {
        let mut key = ForeignKey::create();
        key = key.to_tbl(Alias::new(&self.table));
        for column in self.columns.iter() {
            key = key.from_col(Alias::new(&column));
        }
        for ref_col in self.foreign_columns.iter() {
            key = key.to_col(Alias::new(&ref_col));
        }
        // key = key.on_update(match self.on_update {
        //     ForeignKeyAction::Cascade => sea_query::ForeignKeyAction::Cascade,
        //     ForeignKeyAction::SetNull => sea_query::ForeignKeyAction::SetNull,
        //     ForeignKeyAction::SetDefault => sea_query::ForeignKeyAction::SetDefault,
        //     ForeignKeyAction::Restrict => sea_query::ForeignKeyAction::Restrict,
        //     ForeignKeyAction::NoAction => sea_query::ForeignKeyAction::NoAction,
        // });
        // key = key.on_delete(match self.on_delete {
        //     ForeignKeyAction::Cascade => sea_query::ForeignKeyAction::Cascade,
        //     ForeignKeyAction::SetNull => sea_query::ForeignKeyAction::SetNull,
        //     ForeignKeyAction::SetDefault => sea_query::ForeignKeyAction::SetDefault,
        //     ForeignKeyAction::Restrict => sea_query::ForeignKeyAction::Restrict,
        //     ForeignKeyAction::NoAction => sea_query::ForeignKeyAction::NoAction,
        // });
        key
    }
}
