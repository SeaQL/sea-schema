use crate::mysql::def::{ColumnInfo, Type};
use sea_query::{escape_string, Alias, ColumnDef};
use std::fmt::Write;

impl ColumnInfo {
    pub fn write(&self) -> ColumnDef {
        let mut col_def = ColumnDef::new(Alias::new(self.name.as_str()));
        self.write_col_type(&mut col_def);
        if !self.null {
            col_def = col_def.not_null();
        }
        if self.extra.auto_increment {
            col_def = col_def.auto_increment();
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

    pub fn write_col_type(&self, col_def: &mut ColumnDef) {
        match self.col_type {
            Type::Serial => todo!(),
            Type::Bit(_) => todo!(),
            Type::TinyInt(_) => todo!(),
            Type::Bool => todo!(),
            Type::SmallInt(_) => todo!(),
            Type::MediumInt(_) => todo!(),
            Type::Int(_) => todo!(),
            Type::BigInt(_) => todo!(),
            Type::Decimal(_) => todo!(),
            Type::Float(_) => todo!(),
            Type::Double(_) => todo!(),
            Type::Date => todo!(),
            Type::Time(_) => todo!(),
            Type::DateTime(_) => todo!(),
            Type::Timestamp(_) => todo!(),
            Type::Year => todo!(),
            Type::Char(_) => todo!(),
            Type::NChar(_) => todo!(),
            Type::Varchar(_) => todo!(),
            Type::NVarchar(_) => todo!(),
            Type::Binary(_) => todo!(),
            Type::Varbinary(_) => todo!(),
            Type::Text(_) => todo!(),
            Type::TinyText(_) => todo!(),
            Type::MediumText(_) => todo!(),
            Type::LongText(_) => todo!(),
            Type::Blob(_) => todo!(),
            Type::TinyBlob => todo!(),
            Type::MediumBlob => todo!(),
            Type::LongBlob => todo!(),
            Type::Enum(_) => todo!(),
            Type::Set(_) => todo!(),
            Type::Geometry(_) => todo!(),
            Type::Point(_) => todo!(),
            Type::LineString(_) => todo!(),
            Type::Polygon(_) => todo!(),
            Type::MultiPoint(_) => todo!(),
            Type::MultiLineString(_) => todo!(),
            Type::MultiPolygon(_) => todo!(),
            Type::GeometryCollection(_) => todo!(),
            Type::Json => todo!(),
            Type::Unknown(_) => todo!(),
        }
    }
}
