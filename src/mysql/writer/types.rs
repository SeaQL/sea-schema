use crate::mysql::def::{
    BlobAttr, EnumDef, GeometryAttr, NumericAttr, SetDef, StringAttr, TimeAttr, Type,
};
use sea_query::{EscapeBuilder, Iden, MysqlQueryBuilder};
use std::{borrow::Cow, fmt::Write};

impl Iden for Type {
    fn quoted(&self) -> Cow<'static, str> {
        let mut string = self.unquoted().to_owned();
        let s = &mut string;
        match self {
            Self::Serial => (),
            Self::Bit(attr) => {
                Self::write_numeric_attr(s, attr);
            }
            Self::TinyInt(attr) => {
                Self::write_numeric_attr(s, attr);
            }
            Self::Bool => (),
            Self::SmallInt(attr) => {
                Self::write_numeric_attr(s, attr);
            }
            Self::MediumInt(attr) => {
                Self::write_numeric_attr(s, attr);
            }
            Self::Int(attr) => {
                Self::write_numeric_attr(s, attr);
            }
            Self::BigInt(attr) => {
                Self::write_numeric_attr(s, attr);
            }
            Self::Decimal(attr) => {
                Self::write_numeric_attr(s, attr);
            }
            Self::Float(attr) => {
                Self::write_numeric_attr(s, attr);
            }
            Self::Double(attr) => {
                Self::write_numeric_attr(s, attr);
            }
            Self::Date => (),
            Self::Time(attr) => {
                Self::write_time_attr(s, attr);
            }
            Self::DateTime(attr) => {
                Self::write_time_attr(s, attr);
            }
            Self::Timestamp(attr) => {
                Self::write_time_attr(s, attr);
            }
            Self::Year => (),
            Self::Char(attr) => {
                Self::write_string_attr(s, attr);
            }
            Self::NChar(attr) => {
                Self::write_string_attr(s, attr);
            }
            Self::Varchar(attr) => {
                Self::write_string_attr(s, attr);
            }
            Self::NVarchar(attr) => {
                Self::write_string_attr(s, attr);
            }
            Self::Binary(attr) => {
                Self::write_string_attr(s, attr);
            }
            Self::Varbinary(attr) => {
                Self::write_string_attr(s, attr);
            }
            Self::Text(attr) => {
                Self::write_string_attr(s, attr);
            }
            Self::TinyText(attr) => {
                Self::write_string_attr(s, attr);
            }
            Self::MediumText(attr) => {
                Self::write_string_attr(s, attr);
            }
            Self::LongText(attr) => {
                Self::write_string_attr(s, attr);
            }
            Self::Blob(attr) => {
                Self::write_blob_attr(s, attr);
            }
            Self::TinyBlob => (),
            Self::MediumBlob => (),
            Self::LongBlob => (),
            Self::Enum(def) => {
                Self::write_enum_def(s, def);
            }
            Self::Set(def) => {
                Self::write_set_def(s, def);
            }
            Self::Geometry(attr) => {
                Self::write_geometry_attr(s, attr);
            }
            Self::Point(attr) => {
                Self::write_geometry_attr(s, attr);
            }
            Self::LineString(attr) => {
                Self::write_geometry_attr(s, attr);
            }
            Self::Polygon(attr) => {
                Self::write_geometry_attr(s, attr);
            }
            Self::MultiPoint(attr) => {
                Self::write_geometry_attr(s, attr);
            }
            Self::MultiLineString(attr) => {
                Self::write_geometry_attr(s, attr);
            }
            Self::MultiPolygon(attr) => {
                Self::write_geometry_attr(s, attr);
            }
            Self::GeometryCollection(attr) => {
                Self::write_geometry_attr(s, attr);
            }
            Self::Json => (),
            Self::Unknown(_) => (),
        }
        Cow::Owned(string)
    }

    fn unquoted(&self) -> &str {
        match self {
            Self::Serial => "BIGINT UNSIGNED NOT NULL AUTO_INCREMENT UNIQUE",
            Self::Bit(_) => "BIT",
            Self::TinyInt(_) => "TINYINT",
            Self::Bool => "BOOL",
            Self::SmallInt(_) => "SMALLINT",
            Self::MediumInt(_) => "MEDIUMINT",
            Self::Int(_) => "INT",
            Self::BigInt(_) => "BIGINT",
            Self::Decimal(_) => "DECIMAL",
            Self::Float(_) => "FLOAT",
            Self::Double(_) => "DOUBLE",
            Self::Date => "DATE",
            Self::Time(_) => "TIME",
            Self::DateTime(_) => "DATETIME",
            Self::Timestamp(_) => "TIMESTAMP",
            Self::Year => "YEAR",
            Self::Char(_) => "CHAR",
            Self::NChar(_) => "NCHAR",
            Self::Varchar(_) => "VARCHAR",
            Self::NVarchar(_) => "NVARCHAR",
            Self::Binary(_) => "BINARY",
            Self::Varbinary(_) => "VARBINARY",
            Self::Text(_) => "TEXT",
            Self::TinyText(_) => "TINYTEXT",
            Self::MediumText(_) => "MEDIUMTEXT",
            Self::LongText(_) => "LONGTEXT",
            Self::Blob(_) => "BLOB",
            Self::TinyBlob => "TINYBLOB",
            Self::MediumBlob => "MEDIUMBLOB",
            Self::LongBlob => "LONGBLOB",
            Self::Enum(_) => "ENUM",
            Self::Set(_) => "SET",
            Self::Geometry(_) => "GEOMETRY",
            Self::Point(_) => "POINT",
            Self::LineString(_) => "LINESTRING",
            Self::Polygon(_) => "POLYGON",
            Self::MultiPoint(_) => "MULTIPOINT",
            Self::MultiLineString(_) => "MULTILINESTRING",
            Self::MultiPolygon(_) => "MULTIPOLYGON",
            Self::GeometryCollection(_) => "GEOMETRYCOLLECTION",
            Self::Json => "JSON",
            Self::Unknown(string) => string,
        }
    }
}

impl Type {
    pub fn write_numeric_attr(s: &mut String, num: &NumericAttr) {
        if num.maximum.is_some() || num.decimal.is_some() {
            write!(s, "(").unwrap();
        }
        if num.maximum.is_some() {
            write!(s, "{}", num.maximum.unwrap()).unwrap();
        }
        if num.maximum.is_some() && num.decimal.is_some() {
            write!(s, ", ").unwrap();
        }
        if num.decimal.is_some() {
            write!(s, "{}", num.decimal.unwrap()).unwrap();
        }
        if num.maximum.is_some() || num.decimal.is_some() {
            write!(s, ")").unwrap();
        }
        if num.unsigned.is_some() && num.unsigned.unwrap() {
            write!(s, " UNSIGNED").unwrap();
        }
        if num.zero_fill.is_some() && num.zero_fill.unwrap() {
            write!(s, " ZEROFILL").unwrap();
        }
    }

    pub fn write_time_attr(s: &mut String, attr: &TimeAttr) {
        if attr.fractional.is_some() {
            write!(s, "({})", attr.fractional.unwrap()).unwrap();
        }
    }

    pub fn write_string_attr(s: &mut String, attr: &StringAttr) {
        if attr.length.is_some() {
            write!(s, "({})", attr.length.unwrap()).unwrap();
        }
        if let Some(charset) = &attr.charset {
            write!(s, " CHARACTER SET {}", charset.unquoted()).unwrap();
        }
        if let Some(collation) = &attr.collation {
            write!(s, " COLLATE {}", collation.unquoted()).unwrap();
        }
    }

    pub fn write_blob_attr(s: &mut String, attr: &BlobAttr) {
        if attr.length.is_some() {
            write!(s, "({})", attr.length.unwrap()).unwrap();
        }
    }

    pub fn write_enum_def(s: &mut String, def: &EnumDef) {
        write!(s, " (").unwrap();
        for (i, val) in def.values.iter().enumerate() {
            if i > 0 {
                write!(s, ", ").unwrap();
            }
            write!(s, "\'{}\'", MysqlQueryBuilder.escape_string(val.as_str())).unwrap();
        }
        write!(s, ")").unwrap();
        Self::write_string_attr(s, &def.attr);
    }

    pub fn write_set_def(s: &mut String, def: &SetDef) {
        write!(s, " (").unwrap();
        for (i, val) in def.members.iter().enumerate() {
            if i > 0 {
                write!(s, ", ").unwrap();
            }
            write!(s, "\'{}\'", MysqlQueryBuilder.escape_string(val.as_str())).unwrap();
        }
        write!(s, ")").unwrap();
        Self::write_string_attr(s, &def.attr);
    }

    pub fn write_geometry_attr(s: &mut String, attr: &GeometryAttr) {
        if attr.srid.is_some() {
            write!(s, " SRID {}", attr.srid.unwrap()).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mysql::def::{CharSet, Collation};

    #[test]
    fn test_1() {
        assert_eq!(
            Type::Serial.quoted().as_ref(),
            "BIGINT UNSIGNED NOT NULL AUTO_INCREMENT UNIQUE"
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Type::Bit(NumericAttr::m(1)).quoted().as_ref(), "BIT(1)");
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Type::TinyInt(NumericAttr::default().unsigned().take())
                .quoted()
                .as_ref(),
            "TINYINT UNSIGNED"
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Type::TinyInt(NumericAttr::default().unsigned().zero_fill().take())
                .quoted()
                .as_ref(),
            "TINYINT UNSIGNED ZEROFILL"
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(Type::Bool.quoted().as_ref(), "BOOL");
    }

    #[test]
    fn test_6() {
        assert_eq!(
            Type::SmallInt(NumericAttr::m(8)).quoted().as_ref(),
            "SMALLINT(8)"
        );
    }

    #[test]
    fn test_7() {
        assert_eq!(Type::Int(NumericAttr::m(11)).quoted().as_ref(), "INT(11)");
    }

    #[test]
    fn test_8() {
        assert_eq!(
            Type::Int(NumericAttr::m(11).unsigned().take())
                .quoted()
                .as_ref(),
            "INT(11) UNSIGNED"
        );
    }

    #[test]
    fn test_9() {
        assert_eq!(
            Type::BigInt(NumericAttr::m(22)).quoted().as_ref(),
            "BIGINT(22)"
        );
    }

    #[test]
    fn test_10() {
        assert_eq!(
            Type::Decimal(NumericAttr::m_d(12, 8)).quoted().as_ref(),
            "DECIMAL(12, 8)"
        );
    }

    #[test]
    fn test_11() {
        assert_eq!(
            Type::Decimal(NumericAttr::m(4)).quoted().as_ref(),
            "DECIMAL(4)"
        );
    }

    #[test]
    fn test_12() {
        assert_eq!(
            Type::Float(NumericAttr::default()).quoted().as_ref(),
            "FLOAT"
        );
        assert_eq!(
            Type::Double(NumericAttr::default()).quoted().as_ref(),
            "DOUBLE"
        );
    }

    #[test]
    fn test_13() {
        assert_eq!(Type::Time(TimeAttr::default()).quoted().as_ref(), "TIME");
        assert_eq!(Type::Time(TimeAttr::fsp(6)).quoted().as_ref(), "TIME(6)");
    }

    #[test]
    fn test_14() {
        assert_eq!(
            Type::DateTime(TimeAttr::default()).quoted().as_ref(),
            "DATETIME"
        );
        assert_eq!(
            Type::Timestamp(TimeAttr::default()).quoted().as_ref(),
            "TIMESTAMP"
        );
        assert_eq!(Type::Year.quoted().as_ref(), "YEAR");
    }

    #[test]
    fn test_15() {
        assert_eq!(Type::Char(StringAttr::default()).quoted().as_ref(), "CHAR");
        assert_eq!(
            Type::NChar(StringAttr::default()).quoted().as_ref(),
            "NCHAR"
        );
        assert_eq!(
            Type::Varchar(StringAttr::default()).quoted().as_ref(),
            "VARCHAR"
        );
        assert_eq!(
            Type::NVarchar(StringAttr::default()).quoted().as_ref(),
            "NVARCHAR"
        );
        assert_eq!(
            Type::Binary(StringAttr::default()).quoted().as_ref(),
            "BINARY"
        );
        assert_eq!(
            Type::Varbinary(StringAttr::default()).quoted().as_ref(),
            "VARBINARY"
        );
        assert_eq!(Type::Text(StringAttr::default()).quoted().as_ref(), "TEXT");
        assert_eq!(
            Type::TinyText(StringAttr::default()).quoted().as_ref(),
            "TINYTEXT"
        );
        assert_eq!(
            Type::MediumText(StringAttr::default()).quoted().as_ref(),
            "MEDIUMTEXT"
        );
        assert_eq!(
            Type::LongText(StringAttr::default()).quoted().as_ref(),
            "LONGTEXT"
        );
    }

    #[test]
    fn test_16() {
        assert_eq!(
            Type::Varchar(StringAttr::length(255)).quoted().as_ref(),
            "VARCHAR(255)"
        );
        assert_eq!(
            Type::Varchar(StringAttr {
                length: Some(255),
                charset: Some(CharSet::Utf8Mb4),
                collation: None,
            })
            .quoted()
            .as_ref(),
            "VARCHAR(255) CHARACTER SET utf8mb4"
        );
        assert_eq!(
            Type::Varchar(StringAttr {
                length: Some(255),
                charset: Some(CharSet::Utf8Mb4),
                collation: Some(Collation::Utf8Mb4Bin),
            })
            .quoted()
            .as_ref(),
            "VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin"
        );
    }

    #[test]
    fn test_17() {
        assert_eq!(Type::Blob(BlobAttr::default()).quoted().as_ref(), "BLOB");
        assert_eq!(
            Type::Blob(BlobAttr::length(128)).quoted().as_ref(),
            "BLOB(128)"
        );
        assert_eq!(Type::TinyBlob.quoted().as_ref(), "TINYBLOB");
        assert_eq!(Type::MediumBlob.quoted().as_ref(), "MEDIUMBLOB");
        assert_eq!(Type::LongBlob.quoted().as_ref(), "LONGBLOB");
    }

    #[test]
    fn test_18() {
        assert_eq!(
            Type::Enum(EnumDef {
                values: vec!["A".to_owned(), "B".to_owned(), "C".to_owned()],
                attr: StringAttr::default(),
            })
            .quoted()
            .as_ref(),
            "ENUM ('A', 'B', 'C')"
        );

        assert_eq!(
            Type::Enum(EnumDef {
                values: vec!["A".to_owned(), "B".to_owned(), "C".to_owned()],
                attr: StringAttr {
                    length: None,
                    charset: Some(CharSet::Utf8Mb4),
                    collation: None,
                },
            })
            .quoted()
            .as_ref(),
            "ENUM ('A', 'B', 'C') CHARACTER SET utf8mb4"
        );
    }

    #[test]
    fn test_19() {
        assert_eq!(
            Type::Geometry(GeometryAttr::srid(4326)).quoted().as_ref(),
            "GEOMETRY SRID 4326"
        );
    }

    #[test]
    fn test_20() {
        assert_eq!(
            Type::Geometry(GeometryAttr::default()).quoted().as_ref(),
            "GEOMETRY"
        );
        assert_eq!(
            Type::Point(GeometryAttr::default()).quoted().as_ref(),
            "POINT"
        );
        assert_eq!(
            Type::LineString(GeometryAttr::default()).quoted().as_ref(),
            "LINESTRING"
        );
        assert_eq!(
            Type::Polygon(GeometryAttr::default()).quoted().as_ref(),
            "POLYGON"
        );
        assert_eq!(
            Type::MultiPoint(GeometryAttr::default()).quoted().as_ref(),
            "MULTIPOINT"
        );
        assert_eq!(
            Type::MultiLineString(GeometryAttr::default())
                .quoted()
                .as_ref(),
            "MULTILINESTRING"
        );
        assert_eq!(
            Type::MultiPolygon(GeometryAttr::default())
                .quoted()
                .as_ref(),
            "MULTIPOLYGON"
        );
        assert_eq!(
            Type::GeometryCollection(GeometryAttr::default())
                .quoted()
                .as_ref(),
            "GEOMETRYCOLLECTION"
        );
    }

    #[test]
    fn test_21() {
        assert_eq!(Type::Json.quoted().as_ref(), "JSON");
    }

    #[test]
    fn test_22() {
        assert_eq!(Type::Unknown("hello".to_owned()).quoted().as_ref(), "hello");
        assert_eq!(
            Type::Unknown("world(2)".to_owned()).quoted().as_ref(),
            "world(2)"
        );
    }
}
