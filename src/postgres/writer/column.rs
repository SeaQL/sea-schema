use crate::postgres::def::{ColumnInfo, Type};
use sea_query::{Alias, ColumnDef, PgInterval};
use std::{convert::TryFrom, fmt::Write};

impl ColumnInfo {
    pub fn write(&self) -> ColumnDef {
        let mut col_info = self.clone();
        let mut col_def = ColumnDef::new(Alias::new(self.name.as_str()));
        let mut extras: Vec<String> = Vec::new();
        if let Some(default) = self.default.as_ref() {
            if default.0.starts_with("nextval") {
                col_info = Self::convert_to_serial(col_info);
            } else {
                let mut string = "".to_owned();
                write!(&mut string, "DEFAULT {}", default.0).unwrap();
                extras.push(string);
            }
        }
        if self.is_identity {
            col_info = Self::convert_to_serial(col_info);
        }
        col_def = col_info.write_col_type(col_def);
        if self.not_null.is_some() {
            col_def.not_null();
        }
        if !extras.is_empty() {
            col_def.extra(extras.join(" "));
        }
        col_def
    }

    fn convert_to_serial(mut col_info: ColumnInfo) -> ColumnInfo {
        match col_info.col_type {
            Type::SmallInt => {
                col_info.col_type = Type::SmallSerial;
            }
            Type::Integer => {
                col_info.col_type = Type::Serial;
            }
            Type::BigInt => {
                col_info.col_type = Type::BigSerial;
            }
            _ => {}
        };
        col_info
    }

    pub fn write_col_type(&self, mut col_def: ColumnDef) -> ColumnDef {
        match &self.col_type {
            Type::SmallInt => {
                col_def.small_integer();
            }
            Type::Integer => {
                col_def.integer();
            }
            Type::BigInt => {
                col_def.big_integer();
            }
            Type::Decimal(num_attr) | Type::Numeric(num_attr) => {
                if num_attr.precision.is_none() & num_attr.scale.is_none() {
                    col_def.decimal();
                } else {
                    col_def.decimal_len(
                        num_attr.precision.unwrap_or(0) as u32,
                        num_attr.scale.unwrap_or(0) as u32,
                    );
                }
            }
            Type::Real => {
                col_def.float();
            }
            Type::DoublePrecision => {
                col_def.double();
            }
            Type::SmallSerial => {
                col_def.small_integer().auto_increment();
            }
            Type::Serial => {
                col_def.integer().auto_increment();
            }
            Type::BigSerial => {
                col_def.big_integer().auto_increment();
            }
            Type::Money => {
                col_def.money();
            }
            Type::Varchar(string_attr) => {
                match string_attr.length {
                    Some(length) => col_def.string_len(length.into()),
                    None => col_def.string(),
                };
            }
            Type::Char(string_attr) => {
                match string_attr.length {
                    Some(length) => col_def.char_len(length.into()),
                    None => col_def.char(),
                };
            }
            Type::Text => {
                col_def.text();
            }
            Type::Bytea => {
                col_def.binary();
            }
            Type::Timestamp(time_attr) => {
                match time_attr.precision {
                    Some(precision) => col_def.timestamp_len(precision.into()),
                    None => col_def.timestamp(),
                };
            }
            Type::TimestampWithTimeZone(time_attr) => {
                match time_attr.precision {
                    Some(precision) => col_def.timestamp_with_time_zone_len(precision.into()),
                    None => col_def.timestamp_with_time_zone(),
                };
            }
            Type::Date => {
                col_def.date();
            }
            Type::Time(time_attr) => {
                match time_attr.precision {
                    Some(precision) => col_def.time_len(precision.into()),
                    None => col_def.time(),
                };
            }
            Type::TimeWithTimeZone(time_attr) => {
                match time_attr.precision {
                    Some(precision) => col_def.time_len(precision.into()),
                    None => col_def.time(),
                };
            }
            Type::Interval(interval_attr) => {
                let field = match &interval_attr.field {
                    Some(field) => PgInterval::try_from(field).ok(),
                    None => None,
                };
                let precision = interval_attr.precision.map(Into::into);
                col_def.interval(field, precision);
            }
            Type::Boolean => {
                col_def.boolean();
            }
            Type::Point => {
                col_def.custom(Alias::new("point"));
            }
            Type::Line => {
                col_def.custom(Alias::new("line"));
            }
            Type::Lseg => {
                col_def.custom(Alias::new("lseg"));
            }
            Type::Box => {
                col_def.custom(Alias::new("box"));
            }
            Type::Path => {
                col_def.custom(Alias::new("path"));
            }
            Type::Polygon => {
                col_def.custom(Alias::new("polygon"));
            }
            Type::Circle => {
                col_def.custom(Alias::new("circle"));
            }
            Type::Cidr => {
                col_def.custom(Alias::new("cidr"));
            }
            Type::Inet => {
                col_def.custom(Alias::new("inet"));
            }
            Type::MacAddr => {
                col_def.custom(Alias::new("macaddr"));
            }
            Type::MacAddr8 => {
                col_def.custom(Alias::new("macaddr8"));
            }
            Type::Bit(bit_attr) => {
                let mut str = String::new();
                write!(str, "bit").unwrap();
                if bit_attr.length.is_some() {
                    write!(str, "(").unwrap();
                    if let Some(length) = bit_attr.length {
                        write!(str, "{}", length).unwrap();
                    }
                    write!(str, ")").unwrap();
                }
                col_def.custom(Alias::new(&str));
            }
            Type::TsVector => {
                col_def.custom(Alias::new("tsvector"));
            }
            Type::TsQuery => {
                col_def.custom(Alias::new("tsquery"));
            }
            Type::Uuid => {
                col_def.uuid();
            }
            Type::Xml => {
                col_def.custom(Alias::new("xml"));
            }
            Type::Json => {
                col_def.json();
            }
            Type::JsonBinary => {
                col_def.json_binary();
            }
            Type::Array => {
                col_def.custom(Alias::new("array"));
            }
            Type::Int4Range => {
                col_def.custom(Alias::new("int4range"));
            }
            Type::Int8Range => {
                col_def.custom(Alias::new("int8range"));
            }
            Type::NumRange => {
                col_def.custom(Alias::new("numrange"));
            }
            Type::TsRange => {
                col_def.custom(Alias::new("tsrange"));
            }
            Type::TsTzRange => {
                col_def.custom(Alias::new("tstzrange"));
            }
            Type::DateRange => {
                col_def.custom(Alias::new("daterange"));
            }
            Type::PgLsn => {
                col_def.custom(Alias::new("pg_lsn"));
            }
            Type::Unknown(s) => {
                col_def.custom(Alias::new(s));
            }
            Type::Enum(enum_def) => {
                col_def.enumeration(&enum_def.typename, &enum_def.values);
            }
        };
        col_def
    }
}
