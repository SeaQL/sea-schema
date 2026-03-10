use sea_query::{ColumnType, StringLen};
use std::num::ParseIntError;

pub fn parse_type(data_type: String) -> Result<ColumnType, ParseIntError> {
    let data_type = data_type.as_str();
    let mut type_name = data_type;
    let mut parts: Vec<u32> = Vec::new();
    if let Some((prefix, suffix)) = data_type.split_once('(') {
        if let Some(suffix) = suffix.strip_suffix(')') {
            type_name = prefix;
            for part in suffix.split(',') {
                if let Ok(part) = part.trim().parse() {
                    parts.push(part);
                } else {
                    break;
                }
            }
        }
    }
    Ok(match type_name.to_lowercase().as_str() {
        "char" => ColumnType::Char(parts.into_iter().next()),
        "varchar" => ColumnType::String(match parts.into_iter().next() {
            Some(length) => StringLen::N(length),
            None => StringLen::None,
        }),
        "text" => ColumnType::Text,
        "tinyint" => ColumnType::TinyInteger,
        "smallint" => ColumnType::SmallInteger,
        "integer" => ColumnType::BigInteger,
        "bigint" => ColumnType::BigInteger,
        "float" => ColumnType::Float,
        "double" => ColumnType::Double,
        "real" => {
            if parts.len() == 2 {
                ColumnType::Decimal(Some((parts[0], parts[1])))
            } else {
                ColumnType::Double
            }
        }
        "decimal" | "real_decimal" => ColumnType::Decimal(if parts.len() == 2 {
            Some((parts[0], parts[1]))
        } else {
            None
        }),
        "datetime_text" => ColumnType::DateTime,
        "timestamp" | "timestamp_text" => ColumnType::Timestamp,
        "timestamp_with_timezone_text" => ColumnType::TimestampWithTimeZone,
        "time_text" => ColumnType::Time,
        "date_text" => ColumnType::Date,
        "blob" => {
            if parts.len() == 1 {
                ColumnType::Binary(parts[0])
            } else {
                ColumnType::Blob
            }
        }
        "varbinary_blob" if parts.len() == 1 => {
            ColumnType::VarBinary(match parts.into_iter().next() {
                Some(length) => StringLen::N(length),
                None => StringLen::None,
            })
        }
        "boolean" => ColumnType::Boolean,
        "money" | "real_money" => ColumnType::Money(if parts.len() == 2 {
            Some((parts[0], parts[1]))
        } else {
            None
        }),
        "json_text" => ColumnType::Json,
        "jsonb_text" => ColumnType::JsonBinary,
        "uuid_text" => ColumnType::Uuid,
        type_name => {
            if type_name.contains("int") {
                ColumnType::BigInteger
            } else if type_name.contains("char")
                || type_name.contains("clob")
                || type_name.contains("text")
            {
                ColumnType::Text
            } else if type_name.contains("real")
                || type_name.contains("floa")
                || type_name.contains("doub")
            {
                ColumnType::Double
            } else {
                ColumnType::custom(data_type.to_owned())
            }
        }
    })
}

/// The `hidden` field returned by `PRAGMA table_xinfo`.
#[derive(Debug, PartialEq, Clone)]
pub enum ColumnVisibility {
    /// `0` - ordinary column
    Visible,
    /// `1` - hidden column in a virtual table
    HiddenVirtual,
    /// `2` - generated VIRTUAL column
    GeneratedVirtual,
    /// `3` - generated STORED column
    GeneratedStored,
}

impl ColumnVisibility {
    pub fn from_hidden(value: i32) -> Self {
        match value {
            1 => Self::HiddenVirtual,
            2 => Self::GeneratedVirtual,
            3 => Self::GeneratedStored,
            _ => Self::Visible,
        }
    }

/// The default types for an SQLite `dflt_value`
#[derive(Debug, PartialEq, Clone)]
pub enum DefaultType {
    Integer(i64),
    Float(f32),
    String(String),
    Null,
    Unspecified,
    CurrentTimestamp,
}
