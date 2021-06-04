use crate::mysql::def::{ColumnInfo, ColumnKey};
use sea_query::{escape_string, Alias, ColumnDef};
use std::fmt::Write;

impl ColumnInfo {
    pub fn write(&self) -> ColumnDef {
        let mut col_def =
            ColumnDef::new(Alias::new(self.name.as_str())).custom(self.col_type.clone());
        if !self.null {
            col_def = col_def.not_null();
        }
        if self.extra.auto_increment {
            col_def = col_def.auto_increment();
        }
        if self.key.eq(&ColumnKey::Primary) {
            col_def = col_def.primary_key();
        }
        let mut extras = Vec::new();
        if let Some(default) = self.default.as_ref() {
            let mut string = "".to_owned();
            write!(&mut string, "DEFAULT {}", default.expr).unwrap();
            extras.push(string);
        }
        if self.extra.on_update_current_timestamp {
            extras.push("ON UPDATE CURRENT_TIMESTAMP".to_owned());
        }
        if !self.comment.is_empty() {
            let mut string = "".to_owned();
            write!(&mut string, "COMMENT '{}'", escape_string(&self.comment)).unwrap();
            extras.push(string);
        }
        if !extras.is_empty() {
            col_def = col_def.extra(extras.join(" "));
        }
        col_def
    }
}
