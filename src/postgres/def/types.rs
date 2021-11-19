use sea_query::{backend::PostgresQueryBuilder, extension::postgres::TypeCreateStatement};

use crate::sqlx_types::{postgres::PgRow, FromRow, Row};

#[cfg(feature = "with-serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
/// All built-in types of PostgreSQL, excluding synonyms
pub enum Type {
    // Numeric types
    /// 16 bit integer
    SmallInt,
    /// 32 bit integer
    Integer,
    /// 64 bit integer
    BigInt,
    /// User-specified precision number
    Decimal(ArbitraryPrecisionNumericAttr),
    /// User-specified precision number
    Numeric(ArbitraryPrecisionNumericAttr),
    /// 32 bit floating-point
    Real,
    /// 64 bit floating-point
    DoublePrecision,
    /// 16 bit autoincrementing integer
    SmallSerial,
    /// 32 bit autoincrementing integer
    Serial,
    /// 64 bit autoincrementing integer
    BigSerial,

    /// Currency amount; 64 bits with a fractional precision determined by the database's lc_monetary
    /// setting
    Money,

    // Character types
    /// Variable-length character array with limit
    Varchar(StringAttr),
    /// Fixed-length character array; blank padded
    Char(StringAttr),
    /// Variable, unlimited length character array
    Text,

    /// Variable length binary string
    Bytea,

    // Date/Time types
    /// Date and time
    Timestamp(TimeAttr),
    TimestampWithTimeZone(TimeAttr),
    /// Date without time of day
    Date,
    /// Time without date
    Time(TimeAttr),
    TimeWithTimeZone(TimeAttr),
    /// Time interval
    Interval(IntervalAttr),

    /// One byte boolean value
    Boolean,

    // TODO:
    // /// A type comprised of a static, ordered set of values
    // Enum,

    // Geometric types
    /// Point on a plane
    Point,
    /// Infinite line
    Line,
    /// Finite line segment
    Lseg,
    /// Rectangular box
    Box,
    /// Closed or open path
    Path,
    /// Polygon (similar to a closed path)
    Polygon,
    /// Circle composed of a center point and radius
    Circle,

    // Network address types
    /// IPv4 and IPv6 networks
    Cidr,
    /// IPPv4 and IPv6 hosts and networks
    Inet,
    /// 6 byte MAC address
    MacAddr,
    /// 8 byte MAC address in EUI-64 format
    MacAddr8,

    /// Fixed length bit string
    Bit(BitAttr),

    // Text search types
    /// A sorted list of distinct lexemes which are words that have been normalized to merge different
    /// variants of the same word
    TsVector,
    /// A list of lexemes that are to be searched for, and can be combined using Boolean operators AND,
    /// OR, and NOT, as well as a phrase search operation
    TsQuery,

    /// A universally unique identifier as defined by RFC 4122, ISO 9834-8:2005, and related standards
    Uuid,

    /// XML data checked for well-formedness and with additional support functions
    Xml,

    /// JSON data checked for validity and with additional functions
    Json,

    /// Variable-length multidimensional array
    Array,

    // TODO:
    // /// The structure of a row or record; a list of field names and types
    // Composite,

    // Range types
    /// Range of an integer
    Int4Range,
    /// Range of a bigint
    Int8Range,
    /// Range of a numeric
    NumRange,
    /// Range of a timestamp without time zone
    TsRange,
    /// Range of a timestamp with time zone
    TsTzRange,
    /// Range of a date
    DateRange,

    // TODO:
    // /// A user-defined data type that is based on another underlying type with optional constraints
    // /// that restrict valid values
    // Domain,

    // TODO: Object identifier types
    /// A log sequence number
    PgLsn,
    // TODO: Pseudo-types
    Unknown(String),
    /// Defines an PostgreSQL
    Enum(EnumDef),
}

impl Type {
    // TODO: Support more types
    pub fn from_str(name: &str) -> Type {
        match name.to_lowercase().as_str() {
            "smallint" | "int2" => Type::SmallInt,
            "integer" | "int" | "int4" => Type::Integer,
            "bigint" | "int8" => Type::BigInt,
            "decimal" => Type::Decimal(ArbitraryPrecisionNumericAttr::default()),
            "numeric" => Type::Numeric(ArbitraryPrecisionNumericAttr::default()),
            "real" | "float4" => Type::Real,
            "double precision" | "double" | "float8" => Type::DoublePrecision,
            "smallserial" | "serial2" => Type::SmallSerial,
            "serial" | "serial4" => Type::Serial,
            "bigserial" | "serial8" => Type::BigSerial,
            "money" => Type::Money,
            "character varying" | "varchar" => Type::Varchar(StringAttr::default()),
            "character" | "char" => Type::Char(StringAttr::default()),
            "text" => Type::Text,
            "bytea" => Type::Bytea,
            "timestamp" | "timestamp without time zone" => Type::Timestamp(TimeAttr::default()),
            "timestamp with time zone" => Type::TimestampWithTimeZone(TimeAttr::default()),
            "date" => Type::Date,
            "time" | "time without time zone" => Type::Time(TimeAttr::default()),
            "time with time zone" => Type::TimeWithTimeZone(TimeAttr::default()),
            "interval" => Type::Interval(IntervalAttr::default()),
            "boolean" => Type::Boolean,
            // "" => Type::Enum,
            "point" => Type::Point,
            "line" => Type::Line,
            "lseg" => Type::Lseg,
            "box" => Type::Box,
            "path" => Type::Path,
            "polygon" => Type::Polygon,
            "circle" => Type::Circle,
            "cidr" => Type::Cidr,
            "inet" => Type::Inet,
            "macaddr" => Type::MacAddr,
            "macaddr8" => Type::MacAddr8,
            "bit" => Type::Bit(BitAttr::default()),
            "tsvector" => Type::TsVector,
            "tsquery" => Type::TsQuery,
            "uuid" => Type::Uuid,
            "xml" => Type::Xml,
            "json" => Type::Json,
            "array" => Type::Array,
            // "" => Type::Composite,
            "int4range" => Type::Int4Range,
            "int8range" => Type::Int8Range,
            "numrange" => Type::NumRange,
            "tsrange" => Type::TsRange,
            "tstzrange" => Type::TsTzRange,
            "daterange" => Type::DateRange,
            // "" => Type::Domain,
            "pg_lsn" => Type::PgLsn,
            "enum" => Type::Enum(EnumDef::default()),

            _ => Type::Unknown(name.to_owned()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
/// The precision (number of significan digits) and scale (the number of digits in the fractional
/// portion) of an arbitrary precision number (numeric or decimal). When both the precision and
/// scale are not set, any precision or scale up to the implementation limit may be stored.
pub struct ArbitraryPrecisionNumericAttr {
    /// The number of significant digits in the number; a maximum of 1000 when specified
    pub precision: Option<u16>,
    /// The count of decimal digits in the fractional part; integers have a scale of 0
    pub scale: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct StringAttr {
    pub length: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct TimeAttr {
    pub precision: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct IntervalAttr {
    pub field: Option<String>,
    pub precision: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct BitAttr {
    pub length: Option<u16>,
}

impl Type {
    pub fn has_numeric_attr(&self) -> bool {
        matches!(self, Type::Numeric(_) | Type::Decimal(_))
    }

    pub fn has_string_attr(&self) -> bool {
        matches!(self, Type::Varchar(_) | Type::Char(_))
    }

    pub fn has_time_attr(&self) -> bool {
        matches!(
            self,
            Type::Timestamp(_)
                | Type::TimestampWithTimeZone(_)
                | Type::Time(_)
                | Type::TimeWithTimeZone(_)
        )
    }

    pub fn has_interval_attr(&self) -> bool {
        matches!(self, Type::Interval(_))
    }

    pub fn has_bit_attr(&self) -> bool {
        matches!(self, Type::Bit(_))
    }
    /// Get an immutable reference to the [EnumDef] of type [Type::Enum]
    pub fn get_enum_def(&self) -> &EnumDef {
        match self {
            Type::Enum(def) => def,
            _ => panic!("type error"),
        }
    }

    /// Get a mutable reference to the [EnumDef] of type [Type::Enum]
    pub fn get_enum_def_mut(&mut self) -> &mut EnumDef {
        match self {
            Type::Enum(def) => def,
            _ => panic!("type error"),
        }
    }

    /// Is the type given an enum
    pub fn is_enum(&self) -> bool {
        matches!(self, Type::Enum(_))
    }

    /// Returns nome if the type is not an enum `Type::Enum(EnumDef)`
    pub fn enum_to_create_statement(&self) -> Option<String> {
        match self {
            Type::Enum(enum_def) => Some(enum_def.to_sql_query()),
            _ => None,
        }
    }
    /// Convert a raw SQL query into a type [Type::Enum]
    pub fn enum_from_query(query: &str) -> Type {
        use crate::parser::Parser;
        use sea_query::{unescape_string, Token};

        let mut parser = Parser::new(&query);

        let mut type_enum = Type::Enum(EnumDef::default());

        let mut enum_name = String::default();

        while let Some(_) = parser.next() {
            if parser.last == Some(Token::Unquoted("TYPE".to_owned())) && enum_name.is_empty() {
                match parser.curr() {
                    None => (),
                    Some(token) => match token {
                        Token::Quoted(_) => {
                            if let Some(unquoted) = token.unquote() {
                                enum_name = unquoted;
                            }
                        }
                        Token::Unquoted(unquoted_token) => enum_name = unquoted_token.to_owned(),
                        _ => (),
                    },
                };
            }

            if parser.next_if_punctuation("(") {
                while parser.curr().is_some() {
                    if let Some(word) = parser.next_if_quoted_any() {
                        type_enum
                            .get_enum_def_mut()
                            .values
                            .push(unescape_string(word.unquote().unwrap().as_str()));
                        parser.next_if_punctuation(",");
                    } else if parser.curr_is_unquoted() {
                        todo!("there can actually be numeric enum values but is very confusing");
                    }
                    if parser.next_if_punctuation(")") {
                        break;
                    }
                }
            }
        }

        let attr_len = enum_name.len() as u16;

        type_enum.get_enum_def_mut().typename = enum_name;
        type_enum.get_enum_def_mut().attr.length = Some(attr_len);

        type_enum
    }
}

/// Used to ensure enum names and enum fields always implement [sea_query::types::IntoIden]
#[derive(Debug)]
pub struct EnumIden(pub String);

impl sea_query::Iden for EnumIden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "{}", self.0).unwrap();
    }
}

/// Defines an enum for the PostgreSQL module
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct EnumDef {
    /// Holds the fields of the `ENUM`
    pub values: Vec<String>,
    /// Defines the length of the name describing the [Type::Enum]
    pub attr: StringAttr,
    /// Defines the name of the PostgreSQL enum identifier
    pub typename: String,
}

impl EnumDef {
    /// Implements [sea_query::types::IntoIden] for the Enum name
    pub fn typename_impl_iden(&self) -> EnumIden {
        EnumIden(self.typename.to_owned())
    }

    /// Implements [sea_query::types::IntoIden] for the Enum fields
    pub fn values_impl_iden(&self) -> Vec<EnumIden> {
        self.values
            .iter()
            .map(|iden| EnumIden(iden.to_owned()))
            .collect::<Vec<EnumIden>>()
    }

    /// Converts the [EnumDef] to a [TypeCreateStatement]
    pub fn to_create_statement(&self) -> TypeCreateStatement {
        sea_query::extension::postgres::Type::create()
            .as_enum(self.typename_impl_iden())
            .values(self.values_impl_iden())
            .clone()
    }

    /// Converts the [EnumDef] to a SQL statement
    pub fn to_sql_query(&self) -> String {
        sea_query::extension::postgres::Type::create()
            .as_enum(self.typename_impl_iden())
            .values(self.values_impl_iden())
            .to_string(PostgresQueryBuilder)
    }
}

/// Holds the enum and their values from a `PgRow`
#[derive(Debug, FromRow)]
pub struct EnumRow {
    // The name of the enum type
    pub name: String,
    /// The values of the enum type which are concatenated using ` | ` symbol
    /// for example `"sans|serif|monospace"`
    pub values: String,
}

impl From<&PgRow> for EnumRow {
    fn from(row: &PgRow) -> Self {
        EnumRow {
            name: row.get(0),
            values: row.get(1),
        }
    }
}

impl From<&EnumRow> for EnumDef {
    fn from(row: &EnumRow) -> Self {
        let fields = row
            .values
            .split("|")
            .map(|field| field.to_owned())
            .collect::<Vec<String>>();

        Self {
            typename: row.name.to_owned(),
            attr: StringAttr {
                length: Some(row.name.len() as u16),
            },
            values: fields,
        }
    }
}
