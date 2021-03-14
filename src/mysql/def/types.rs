#[derive(Debug)]
pub enum Type {
    Serial(NumericAttr),
    Bit(NumericAttr),
    TinyInt(NumericAttr),
    Bool(NumericAttr),
    SmallInt(NumericAttr),
    MediumInt(NumericAttr),
    Int(NumericAttr),
    BigInt(NumericAttr),
    Decimal(NumericAttr),
    Float(NumericAttr),
    Double(NumericAttr),
    Date,
    Time(TimeAttr),
    DateTime(TimeAttr),
    Timestamp(TimeAttr),
    Year,
    Char(StringAttr),
    NChar(StringAttr),
    Varchar(StringAttr),
    NVarchar(StringAttr),
    Binary(StringAttr),
    Varbinary(StringAttr),
    Text(StringAttr),
    TinyText(StringAttr),
    MediumText(StringAttr),
    LongText(StringAttr),
    Blob(BinaryAttr),
    TinyBlob,
    MediumBlob,
    LongBlob,
    Enum(EnumDef),
    Set(SetDef),
    Geometry(GeometryAttr),
    Point(GeometryAttr),
    LineString(GeometryAttr),
    Polygon(GeometryAttr),
    MultiPoint(GeometryAttr),
    MultiLineString(GeometryAttr),
    MultiPolygon(GeometryAttr),
    GeometryCollection(GeometryAttr),
    Json,
}

#[derive(Debug, Default)]
pub struct NumericAttr {
    maximum: Option<u32>,
    decimal: Option<u32>,
    unsigned: Option<bool>,
    zero_fill: Option<u32>,
}

#[derive(Debug, Default)]
pub struct TimeAttr {
    fractional: Option<u32>,
}

#[derive(Debug, Default)]
pub struct StringAttr {
    length: Option<u32>,
    charset_name: Option<String>,
    collation_name: Option<String>,
}

#[derive(Debug, Default)]
pub struct BinaryAttr {
    length: Option<u32>,
}

#[derive(Debug, Default)]
pub struct EnumDef {
    values: Vec<String>,
    attr: StringAttr,
}

pub type SetDef = EnumDef;

#[derive(Debug, Default)]
pub struct GeometryAttr {
    srid: Option<u32>,
}

impl Type {
    pub fn is_numeric(&self) -> bool {
        match self {
            Type::Serial(_) => true,
            Type::Bit(_) => true,
            Type::TinyInt(_) => true,
            Type::Bool(_) => true,
            Type::SmallInt(_) => true,
            Type::MediumInt(_) => true,
            Type::Int(_) => true,
            Type::BigInt(_) => true,
            Type::Decimal(_) => true,
            Type::Float(_) => true,
            Type::Double(_) => true,
            _ => false,
        }
    }

    pub fn is_date_time(&self) -> bool {
        match self {
            Type::Date => true,
            Type::Time(_) => true,
            Type::DateTime(_) => true,
            Type::Timestamp(_) => true,
            Type::Year => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Type::Char(_) => true,
            Type::NChar(_) => true,
            Type::Varchar(_) => true,
            Type::NVarchar(_) => true,
            Type::Binary(_) => true,
            Type::Varbinary(_) => true,
            Type::Text(_) => true,
            Type::TinyText(_) => true,
            Type::MediumText(_) => true,
            Type::LongText(_) => true,
            _ => false,
        }
    }

    pub fn is_binary(&self) -> bool {
        match self {
            Type::Blob(_) => true,
            Type::TinyBlob => true,
            Type::MediumBlob => true,
            Type::LongBlob => true,
            _ => false,
        }
    }

    pub fn is_spatial(&self) -> bool {
        match self {
            Type::Geometry(_) => true,
            Type::Point(_) => true,
            Type::LineString(_) => true,
            Type::Polygon(_) => true,
            Type::MultiPoint(_) => true,
            Type::MultiLineString(_) => true,
            Type::MultiPolygon(_) => true,
            Type::GeometryCollection(_) => true,
            _ => false,
        }
    }

    pub fn is_aggregate(&self) -> bool {
        match self {
            Type::Enum(_) => true,
            Type::Set(_) => true,
            _ => false,
        }
    }

    pub fn is_other(&self) -> bool {
        match self {
            Type::Json => true,
            _ => false,
        }
    }
}