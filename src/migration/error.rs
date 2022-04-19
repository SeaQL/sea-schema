/// Migration error
#[derive(Debug, PartialEq)]
pub struct MigrationErr(pub String);

impl std::error::Error for MigrationErr {}

impl std::fmt::Display for MigrationErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
