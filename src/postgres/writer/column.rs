use crate::postgres::def::{ArbitraryPrecisionNumericAttr, ColumnInfo, Type};
use core::num;
use sea_query::{escape_string, Alias, ColumnDef, Iden};
use std::fmt::Write;

impl ColumnInfo {
    pub fn write(&self) -> ColumnDef {
        let mut col_def = ColumnDef::new(Alias::new(self.name.as_str()));
        col_def = self.write_col_type(col_def);
        if !self.not_null.is_some() {
            col_def = col_def.not_null();
        }
        // if self.extra.auto_increment {
        //     col_def = col_def.auto_increment();
        // }
        let mut extras = Vec::new();
        if let Some(default) = self.default.as_ref() {
            let mut string = "".to_owned();
            write!(&mut string, "DEFAULT {}", default.0).unwrap();
            extras.push(string);
        }
        // if self.extra.on_update_current_timestamp {
        //     extras.push("ON UPDATE CURRENT_TIMESTAMP".to_owned());
        // }
        // if !self.comment.is_empty() {
        //     let mut string = "".to_owned();
        //     write!(&mut string, "COMMENT '{}'", escape_string(&self.comment)).unwrap();
        //     extras.push(string);
        // }
        if !extras.is_empty() {
            col_def = col_def.extra(extras.join(" "));
        }
        col_def
    }

    pub fn write_col_type(&self, mut col_def: ColumnDef) -> ColumnDef {
        match &self.col_type {
            Type::SmallInt => {
                col_def = col_def.small_integer();
            }
            Type::Integer => {
                col_def = col_def.integer();
            }
            Type::BigInt => {
                col_def = col_def.big_integer();
            }
            Type::Decimal(num_attr) => {
                if num_attr.precision.is_none() & num_attr.scale.is_none() {
                    col_def = col_def.decimal();
                } else {
                    col_def = col_def.decimal_len(
                        num_attr.precision.unwrap_or(0) as u32,
                        num_attr.scale.unwrap_or(0) as u32,
                    );
                }
            }
            Type::Numeric(num_attr) => {}
            Type::Real => {
                col_def = col_def.float();
            }
            Type::DoublePrecision => {
                col_def = col_def.double();
            }
            Type::SmallSerial => {
                col_def = col_def.small_integer().auto_increment();
            }
            Type::Serial => {
                col_def = col_def.integer().auto_increment();
            }
            Type::BigSerial => {
                col_def = col_def.big_integer().auto_increment();
            }
            Type::Money => {
                col_def = col_def.money();
            }
            Type::Varchar => {}
            Type::Char => {}
            Type::Text => {}
            Type::Bytea => {}
            Type::Timestamp => {}
            Type::Date => {}
            Type::Time => {}
            Type::Interval => {}
            Type::Boolean => {}
            Type::Point => {}
            Type::Line => {}
            Type::Lseg => {}
            Type::Box => {}
            Type::Path => {}
            Type::Polygon => {}
            Type::Circle => {}
            Type::Cidr => {}
            Type::Inet => {}
            Type::MacAddr => {}
            Type::MacAddr8 => {}
            Type::Bit => {}
            Type::TsVector => {}
            Type::TsQuery => {}
            Type::Uuid => {}
            Type::Xml => {}
            Type::Json => {}
            Type::Array => {}
            Type::Int4Range => {}
            Type::Int8Range => {}
            Type::NumRange => {}
            Type::TsRange => {}
            Type::TsTzRange => {}
            Type::DateRange => {}
            Type::PgLsn => {}
            Type::Unknown(s) => {
                col_def = col_def.custom(Alias::new(s));
            }
        };
        col_def
    }
}
