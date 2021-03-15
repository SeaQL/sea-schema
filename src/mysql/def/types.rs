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
    Blob(BlobAttr),
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
    Unknown(String),
}

#[derive(Debug, Default)]
pub struct NumericAttr {
    pub maximum: Option<u32>,
    pub decimal: Option<u32>,
    pub unsigned: Option<bool>,
    pub zero_fill: Option<bool>,
}

#[derive(Debug, Default)]
pub struct TimeAttr {
    pub fractional: Option<u32>,
}

#[derive(Debug, Default)]
pub struct StringAttr {
    pub length: Option<u32>,
    pub charset_name: Option<String>,
    pub collation_name: Option<String>,
}

#[derive(Debug, Default)]
pub struct BlobAttr {
    pub length: Option<u32>,
}

#[derive(Debug, Default)]
pub struct EnumDef {
    pub values: Vec<String>,
    pub attr: StringAttr,
}

#[derive(Debug, Default)]
pub struct SetDef {
    pub members: Vec<String>,
    pub attr: StringAttr,
}

#[derive(Debug, Default)]
pub struct GeometryAttr {
    pub srid: Option<u32>,
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

    pub fn is_date(&self) -> bool {
        match self {
            Type::Date => true,
            Type::Year => true,
            _ => false,
        }
    }

    pub fn is_time(&self) -> bool {
        match self {
            Type::Time(_) => true,
            Type::DateTime(_) => true,
            Type::Timestamp(_) => true,
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

    pub fn is_blob(&self) -> bool {
        match self {
            Type::Blob(_) => true,
            Type::TinyBlob => true,
            Type::MediumBlob => true,
            Type::LongBlob => true,
            _ => false,
        }
    }

    pub fn is_free_size_blob(&self) -> bool {
        match self {
            Type::Blob(_) => true,
            _ => false,
        }
    }

    pub fn is_geometry(&self) -> bool {
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

    pub fn is_enum(&self) -> bool {
        match self {
            Type::Enum(_) => true,
            _ => false,
        }
    }

    pub fn is_set(&self) -> bool {
        match self {
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

    pub fn is_unknown(&self) -> bool {
        match self {
            Type::Unknown(_) => true,
            _ => false,
        }
    }

    pub fn get_numeric_attr_mut(&mut self) -> &mut NumericAttr {
        match self {
            Type::Serial(attr) => attr,
            Type::Bit(attr) => attr,
            Type::TinyInt(attr) => attr,
            Type::Bool(attr) => attr,
            Type::SmallInt(attr) => attr,
            Type::MediumInt(attr) => attr,
            Type::Int(attr) => attr,
            Type::BigInt(attr) => attr,
            Type::Decimal(attr) => attr,
            Type::Float(attr) => attr,
            Type::Double(attr) => attr,
            _ => panic!("type error"),
        }
    }

    pub fn get_time_attr_mut(&mut self) -> &mut TimeAttr {
        match self {
            Type::Time(attr) => attr,
            Type::DateTime(attr) => attr,
            Type::Timestamp(attr) => attr,
            _ => panic!("type error"),
        }
    }

    pub fn get_string_attr_mut(&mut self) -> &mut StringAttr {
        match self {
            Type::Char(attr) => attr,
            Type::NChar(attr) => attr,
            Type::Varchar(attr) => attr,
            Type::NVarchar(attr) => attr,
            Type::Binary(attr) => attr,
            Type::Varbinary(attr) => attr,
            Type::Text(attr) => attr,
            Type::TinyText(attr) => attr,
            Type::MediumText(attr) => attr,
            Type::LongText(attr) => attr,
            _ => panic!("type error"),
        }
    }

    pub fn get_blob_attr_mut(&mut self) -> &mut BlobAttr {
        match self {
            Type::Blob(attr) => attr,
            _ => panic!("type error"),
        }
    }

    pub fn get_enum_def_mut(&mut self) -> &mut EnumDef {
        match self {
            Type::Enum(def) => def,
            _ => panic!("type error"),
        }
    }

    pub fn get_set_def_mut(&mut self) -> &mut SetDef {
        match self {
            Type::Set(def) => def,
            _ => panic!("type error"),
        }
    }

    pub fn get_geometry_attr_mut(&mut self) -> &mut GeometryAttr {
        match self {
            Type::Geometry(attr) => attr,
            Type::Point(attr) => attr,
            Type::LineString(attr) => attr,
            Type::Polygon(attr) => attr,
            Type::MultiPoint(attr) => attr,
            Type::MultiLineString(attr) => attr,
            Type::MultiPolygon(attr) => attr,
            Type::GeometryCollection(attr) => attr,
            _ => panic!("type error"),
        }
    }
}