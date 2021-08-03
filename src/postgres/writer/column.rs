use crate::postgres::def::{ArbitraryPrecisionNumericAttr, ColumnInfo, Type};
use core::num;
use sea_query::{escape_string, Alias, ColumnDef, Iden};
use std::{default, fmt::Write};

impl ColumnInfo {
    pub fn write(&self) -> ColumnDef {
        let mut self_copy = self.clone();
        let mut col_def = ColumnDef::new(Alias::new(self.name.as_str()));
        let mut extras: Vec<String> = Vec::new();
        if let Some(default) = self.default.as_ref() {
            if default.0.starts_with("nextval") {
                match self.col_type {
                    Type::SmallInt => {
                        self_copy.col_type = Type::SmallSerial;
                    }
                    Type::Integer => {
                        self_copy.col_type = Type::Serial;
                    }
                    Type::BigInt => {
                        self_copy.col_type = Type::BigSerial;
                    }
                    _ => {}
                }
            } else {
                let mut string = "".to_owned();
                write!(&mut string, "DEFAULT {}", default.0).unwrap();
                extras.push(string);
            }
        }
        col_def = self_copy.write_col_type(col_def);
        if self.not_null.is_some() {
            col_def = col_def.not_null();
        }
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
            Type::Decimal(num_attr) | Type::Numeric(num_attr) => {
                if num_attr.precision.is_none() & num_attr.scale.is_none() {
                    col_def = col_def.decimal();
                } else {
                    col_def = col_def.decimal_len(
                        num_attr.precision.unwrap_or(0) as u32,
                        num_attr.scale.unwrap_or(0) as u32,
                    );
                }
            }
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
            Type::Varchar(string_attr) => {
                col_def = match string_attr.length {
                    Some(length) => col_def.string_len(length.into()),
                    None => col_def.string(),
                };
            }
            Type::Char(string_attr) => {
                col_def = match string_attr.length {
                    Some(length) => col_def.char_len(length.into()),
                    None => col_def.char(),
                };
            }
            Type::Text => {
                col_def = col_def.text();
            }
            Type::Bytea => {
                col_def = col_def.binary();
            }
            Type::Timestamp(time_attr) => {
                col_def = match time_attr.precision {
                    Some(precision) => col_def.timestamp_len(precision.into()),
                    None => col_def.timestamp(),
                };
            }
            Type::TimestampWithTimeZone(time_attr) => {
                col_def = match time_attr.precision {
                    Some(precision) => col_def.timestamp_len(precision.into()),
                    None => col_def.timestamp(),
                };
            }
            Type::Date => {
                col_def = col_def.date();
            }
            Type::Time(time_attr) => {
                col_def = match time_attr.precision {
                    Some(precision) => col_def.time_len(precision.into()),
                    None => col_def.time(),
                };
            }
            Type::TimeWithTimeZone(time_attr) => {
                col_def = match time_attr.precision {
                    Some(precision) => col_def.time_len(precision.into()),
                    None => col_def.time(),
                };
            }
            Type::Interval(time_attr) => {}
            Type::Boolean => {
                col_def = col_def.boolean();
            }
            Type::Point => {
                col_def = col_def.custom(Alias::new("point"));
            }
            Type::Line => {
                col_def = col_def.custom(Alias::new("line"));
            }
            Type::Lseg => {
                col_def = col_def.custom(Alias::new("lseg"));
            }
            Type::Box => {
                col_def = col_def.custom(Alias::new("box"));
            }
            Type::Path => {
                col_def = col_def.custom(Alias::new("path"));
            }
            Type::Polygon => {
                col_def = col_def.custom(Alias::new("ploygon"));
            }
            Type::Circle => {
                col_def = col_def.custom(Alias::new("circle"));
            }
            Type::Cidr => {
                col_def = col_def.custom(Alias::new("cidr"));
            }
            Type::Inet => {
                col_def = col_def.custom(Alias::new("inet"));
            }
            Type::MacAddr => {
                col_def = col_def.custom(Alias::new("macaddr"));
            }
            Type::MacAddr8 => {
                col_def = col_def.custom(Alias::new("macaddr8"));
            }
            Type::Bit(bit_attr) => {
                let mut str = String::new();
                write!(str, "bit");
                if bit_attr.length.is_some() {
                    write!(str, "(");
                    if let Some(length) = bit_attr.length {
                        write!(str, "{}", length);
                    }
                    write!(str, ")");
                }
                col_def = col_def.custom(Alias::new(&str));
            }
            Type::TsVector => {
                col_def = col_def.custom(Alias::new("tsvector"));
            }
            Type::TsQuery => {
                col_def = col_def.custom(Alias::new("tsquery"));
            }
            Type::Uuid => {
                col_def = col_def.custom(Alias::new("uuid"));
            }
            Type::Xml => {
                col_def = col_def.custom(Alias::new("xml"));
            }
            Type::Json => {
                col_def = col_def.custom(Alias::new("json"));
            }
            Type::Array => {
                col_def = col_def.custom(Alias::new("array"));
            }
            Type::Int4Range => {
                col_def = col_def.custom(Alias::new("int4range"));
            }
            Type::Int8Range => {
                col_def = col_def.custom(Alias::new("int8range"));
            }
            Type::NumRange => {
                col_def = col_def.custom(Alias::new("numrange"));
            }
            Type::TsRange => {
                col_def = col_def.custom(Alias::new("tsrange"));
            }
            Type::TsTzRange => {
                col_def = col_def.custom(Alias::new("tstzrange"));
            }
            Type::DateRange => {
                col_def = col_def.custom(Alias::new("daterange"));
            }
            Type::PgLsn => {
                col_def = col_def.custom(Alias::new("pg_lsn"));
            }
            Type::Unknown(s) => {
                col_def = col_def.custom(Alias::new(s));
            }
        };
        col_def
    }
}
