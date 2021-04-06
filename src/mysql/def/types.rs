#[derive(Clone, Debug, PartialEq)]
/// All built-in types of MySQL, excluding synonyms
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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct NumericAttr {
    pub maximum: Option<u32>,
    pub decimal: Option<u32>,
    pub unsigned: Option<bool>,
    pub zero_fill: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TimeAttr {
    pub fractional: Option<u32>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StringAttr {
    pub length: Option<u32>,
    pub charset_name: Option<String>,
    pub collation_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BlobAttr {
    pub length: Option<u32>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EnumDef {
    pub values: Vec<String>,
    pub attr: StringAttr,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SetDef {
    pub members: Vec<String>,
    pub attr: StringAttr,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GeometryAttr {
    pub srid: Option<u32>,
}

impl Type {
    pub fn is_numeric(&self) -> bool {
        matches!(self,
            Type::Serial(_) |
            Type::Bit(_) |
            Type::TinyInt(_) |
            Type::Bool(_) |
            Type::SmallInt(_) |
            Type::MediumInt(_) |
            Type::Int(_) |
            Type::BigInt(_) |
            Type::Decimal(_) |
            Type::Float(_) |
            Type::Double(_)
        )
    }

    pub fn is_date(&self) -> bool {
        matches!(self,
            Type::Date |
            Type::Year
        )
    }

    pub fn is_time(&self) -> bool {
        matches!(self,
            Type::Time(_) |
            Type::DateTime(_) |
            Type::Timestamp(_)
        )
    }

    pub fn is_string(&self) -> bool {
        matches!(self,
            Type::Char(_) |
            Type::NChar(_) |
            Type::Varchar(_) |
            Type::NVarchar(_) |
            Type::Binary(_) |
            Type::Varbinary(_) |
            Type::Text(_) |
            Type::TinyText(_) |
            Type::MediumText(_) |
            Type::LongText(_)
        )
    }

    pub fn is_blob(&self) -> bool {
        matches!(self,
            Type::Blob(_) |
            Type::TinyBlob |
            Type::MediumBlob |
            Type::LongBlob
        )
    }

    pub fn is_free_size_blob(&self) -> bool {
        matches!(self,
            Type::Blob(_)
        )
    }

    pub fn is_geometry(&self) -> bool {
        matches!(self,
            Type::Geometry(_) |
            Type::Point(_) |
            Type::LineString(_) |
            Type::Polygon(_) |
            Type::MultiPoint(_) |
            Type::MultiLineString(_) |
            Type::MultiPolygon(_) |
            Type::GeometryCollection(_)
        )
    }

    pub fn is_enum(&self) -> bool {
        matches!(self,
            Type::Enum(_)
        )
    }

    pub fn is_set(&self) -> bool {
        matches!(self,
            Type::Set(_)
        )
    }

    pub fn is_other(&self) -> bool {
        matches!(self,
            Type::Json
        )
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self,
            Type::Unknown(_)
        )
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