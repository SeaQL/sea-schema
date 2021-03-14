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

#[derive(Debug)]
pub struct NumericAttr {
    maximum: u32,
    decimal: u32,
    unsigned: bool,
    zero_fill: u32,
}

#[derive(Debug)]
pub struct TimeAttr {
    fractional: u32,
}

#[derive(Debug)]
pub struct StringAttr {
    length: u32,
    charset_name: String,
    collation_name: String,
}

#[derive(Debug)]
pub struct BinaryAttr {
    length: u32,
}

#[derive(Debug)]
pub struct EnumDef {
    values: Vec<String>,
    attr: StringAttr,
}

pub type SetDef = EnumDef;

#[derive(Debug)]
pub struct GeometryAttr {
    srid: u32,
}