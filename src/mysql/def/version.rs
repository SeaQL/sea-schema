#[derive(Debug, Default, PartialEq)]
pub struct Version {
    /// The version number converted to integer using the following formula:
    /// major_version * 10000 + minor_version * 100 + sub_version
    pub number: u32,
    /// The system string. it may be: `0ubuntu0.*` or `MariaDB`
    pub system: String,
    /// Additional suffix
    pub suffix: Vec<String>,
}

impl Version {
    /// Return true if the system is MariaDB
    pub fn is_maria_db(&self) -> bool {
        self.system == "MariaDB"
    }

    /// Return true if the system is not MariaDB
    pub fn is_mysql(&self) -> bool {
        !self.is_maria_db()
    }

    /// Return the version number as string. e.g. 8.0.1
    pub fn number_string(&self) -> String {
        format!("{}.{}.{}", self.number / 10000, self.number / 100 % 100, self.number % 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let mut version = Version::default();
        version.number = 50110;
        assert_eq!(version.number_string(), "5.1.10".to_owned());
    }

    #[test]
    fn test_1() {
        let mut version = Version::default();
        version.number = 80023;
        assert_eq!(version.number_string(), "8.0.23".to_owned());
    }
}