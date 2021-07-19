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
    Varchar,
    /// Fixed-length character array; blank padded
    Char,
    /// Variable, unlimited length character array
    Text,

    /// Variable length binary string
    Bytea,

    // Date/Time types
    /// Date and time
    Timestamp,
    /// Date without time of day
    Date,
    /// Time without date
    Time,
    /// Time interval
    Interval,

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
    Bit,

    // Text search types
    /// A sorted list of distincp lexemes which are words that have been normalized to merge different
    /// variants of the same word
    TsVector,
    /// A list of lexemes that are to be searched for, and can be combined using Boolean operators AND,
    /// OR, and NOT, as well as a phrase search operation
    TsQuery,

    /// A universally unique identifier as defined by RFC 4122, ISO 9834-8:2005, and related standards
    Uuid,

    /// XML data checked for well-formedness and with additonal support functions
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

            _ => Type::Unknown(format!("{} is unknown or unimplemented", name)),
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

impl Type {
    pub fn has_numeric_attr(&self) -> bool {
        matches!(self, Type::Numeric(_) | Type::Decimal(_))
    }
}
