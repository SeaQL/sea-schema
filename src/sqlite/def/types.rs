use sea_query::{BlobSize, ColumnType};
use std::num::ParseIntError;

pub type Type = ColumnType;

pub fn parse_type(data_type: &str) -> Result<ColumnType, ParseIntError> {
    let mut type_name = data_type;
    let mut parts: Vec<u32> = Vec::new();
    if let Some((prefix, suffix)) = data_type.split_once('(') {
        if let Some(suffix) = suffix.strip_suffix(')') {
            type_name = prefix;
            for part in suffix.split(",") {
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
        "varchar" => ColumnType::String(parts.into_iter().next()),
        "text" => ColumnType::Text,
        "tinyint" => ColumnType::TinyInteger,
        "smallint" => ColumnType::SmallInteger,
        "integer" => ColumnType::Integer,
        "bigint" => ColumnType::BigInteger,
        "float" => ColumnType::Float,
        "double" => ColumnType::Double,
        "decimal_text" => ColumnType::Decimal(if parts.len() == 2 {
            Some((parts[0], parts[1]))
        } else {
            None
        }),
        "datetime_text" => ColumnType::DateTime,
        "timestamp_text" => ColumnType::Timestamp,
        "timestamp_with_timezone_text" => ColumnType::TimestampWithTimeZone,
        "time_text" => ColumnType::Time,
        "date_text" => ColumnType::Date,
        "tinyblob" => ColumnType::Binary(BlobSize::Tiny),
        "mediumblob" => ColumnType::Binary(BlobSize::Medium),
        "longblob" => ColumnType::Binary(BlobSize::Long),
        "blob" => ColumnType::Binary(BlobSize::Blob(parts.into_iter().next())),
        "varbinary_blob" if parts.len() == 1 => ColumnType::VarBinary(parts[0]),
        "boolean" => ColumnType::Boolean,
        "money" => ColumnType::Money(if parts.len() == 2 {
            Some((parts[0], parts[1]))
        } else {
            None
        }),
        "json_text" => ColumnType::Json,
        "jsonb_text" => ColumnType::JsonBinary,
        "uuid_text" => ColumnType::Uuid,
        _ => ColumnType::custom(data_type),
    })
}

/// The default types for an SQLite `dflt_value`
#[derive(Debug, PartialEq, Clone)]
pub enum DefaultType {
    Integer(i32),
    Float(f32),
    String(String),
    Null,
    Unspecified,
    CurrentTimestamp,
}
