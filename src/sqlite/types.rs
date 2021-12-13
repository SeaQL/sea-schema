use std::num::ParseIntError;

/// A list of the offical SQLite types as outline at the official [SQLite Docs](https://www.sqlite.org/datatype3.html)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    Int,
    Integer,
    TinyInt,
    SmallInt,
    MediumInt,
    BigInt,
    UnsignedBigInt,
    Int2,
    Int8,
    Character { length: u8 },
    VarChar { length: u8 },
    VaryingCharacter { length: u8 },
    Nchar { length: u8 },
    NativeCharacter { length: u8 },
    NvarChar { length: u8 },
    Text,
    Clob,
    Blob, //No datatype specified
    Real,
    Double,
    DoublePrecision,
    Float,
    Numeric,
    Decimal { integral: u8, fractional: u8 },
    Boolean,
    Date,
    DateTime,
}

impl Type {
    /// Maps a string type from an `SqliteRow` into a [Type]
    pub fn to_type(data_type: &str) -> Result<Type, ParseIntError> {
        let data_type = data_type.to_uppercase();

        let split_type: Vec<&str> = data_type.split('(').collect();
        let type_result = match split_type[0] {
            "INT" => Type::Int,
            "INTEGER" => Type::Integer,
            "TINY INT" => Type::TinyInt,
            "SMALL INT" => Type::SmallInt,
            "MEDIUM INT" => Type::MediumInt,
            "BIG INT" => Type::BigInt,
            "UNSIGNED INT" => Type::UnsignedBigInt,
            "INT2" => Type::Int2,
            "INT8" => Type::Int8,
            "TEXT" => Type::Text,
            "CLOB" => Type::Clob,
            "BLOB" => Type::Blob,
            "REAL" => Type::Real,
            "DOUBLE" => Type::Double,
            "DOUBLE PRECISION" => Type::DoublePrecision,
            "FLOAT" => Type::Float,
            "NUMERIC" => Type::Numeric,
            "DECIMAL" => {
                let decimals = split_type[1].chars().collect::<Vec<_>>();

                let integral = decimals[0].to_string().parse::<u8>()?;
                let fractional = decimals[2].to_string().parse::<u8>()?;

                Type::Decimal {
                    integral,
                    fractional,
                }
            }
            "BOOLEAN" => Type::Boolean,
            "DATE" => Type::Date,
            "DATETIME" => Type::DateTime,
            _ => Type::variable_types(&split_type)?,
        };

        Ok(type_result)
    }

    /// Map a [Type] into a [String] in order to pass into a column when invoking `ColumnDef`
    pub fn stringify_type(&self) -> String {
        match self {
            Self::Int => "INT".into(),
            Self::Integer => "INTEGER".into(),
            Self::TinyInt => "TINY INT".into(),
            Self::SmallInt => "SMALL INT".into(),
            Self::MediumInt => "MEDIUM INT".into(),
            Self::BigInt => "BIG INT".into(),
            Self::UnsignedBigInt => "UNSIGNED BIG INT".into(),
            Self::Int2 => "INT2".into(),
            Self::Int8 => "INT8".into(),
            Self::Character { length } => self.concat_type("CHARACTER", length),
            Self::VarChar { length } => self.concat_type("VARCHAR", length),
            Self::VaryingCharacter { length } => self.concat_type("VARYING CHARACTER", length),
            Self::Nchar { length } => self.concat_type("NCHAR", length),
            Self::NativeCharacter { length } => self.concat_type("NATIVE CHARACTER", length),
            Self::NvarChar { length } => self.concat_type("NVARCHAR", length),
            Self::Text => "TEXT".into(),
            Self::Clob => "CLOB".into(),
            Self::Blob => "BLOB".into(), //No datatype specified
            Self::Real => "REAL".into(),
            Self::Double => "DOUBLE".into(),
            Self::DoublePrecision => "DOUBLE PRECISION".into(),
            Self::Float => "FLOAT".into(),
            Self::Numeric => "NUMERIC".into(),
            Self::Decimal {
                integral,
                fractional,
            } => {
                let mut value = String::default();
                value.push_str("DECIMAL");
                value.push('(');
                value.push_str(&integral.to_string());
                value.push(',');
                value.push_str(&fractional.to_string());
                value.push(')');

                value
            }
            Self::Boolean => "BOOLEAN".into(),
            Self::Date => "DATE".into(),
            Self::DateTime => "DATETIME".into(),
        }
    }

    fn concat_type(&self, type_name: &str, length: &u8) -> String {
        let mut value = String::default();
        value.push_str(type_name);
        value.push('(');
        value.push_str(&length.to_string());
        value.push(')');

        value
    }

    fn variable_types(split_type: &[&str]) -> Result<Type, ParseIntError> {
        let length = if !split_type.len() == 1 {
            let maybe_size = split_type[1].replace(')', "");
            maybe_size.parse::<u8>()?
        } else {
            255_u8
        };

        let type_result = match split_type[0] {
            "VARCHAR" => Type::VarChar { length },
            "CHARACTER" => Type::Character { length },
            "VARYING CHARACTER" => Type::VaryingCharacter { length },
            "NCHAR" => Type::Nchar { length },
            "NATIVE CHARACTER" => Type::NativeCharacter { length },
            "NVARCHAR" => Type::NvarChar { length },
            _ => Type::Blob,
        };
        Ok(type_result)
    }
}

/// The default types for an SQLite `dflt_value`
#[derive(Debug, PartialEq, Clone)]
pub enum DefaultType {
    Integer(i32),
    Float(f32),
    String(String),
    Null,
    Unspecified, //FIXME For other types
}
