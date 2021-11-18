use std::num::{ParseFloatError, ParseIntError};

/// This type simplifies error handling
pub type DiscoveryResult<T> = Result<T, SqliteDiscoveryError>;

/// All the errors that can be encountered when using this module
#[derive(Debug)]
pub enum SqliteDiscoveryError {
    /// An error parsing a string from the result of an SQLite query into an rust-language integer
    ParseIntError,
    /// An error parsing a string from the result of an SQLite query into an rust-language float
    ParseFloatError,
    /// The error as defined in [sqlx::Error]
    SqlxError(sqlx::Error),
    /// An operation to discover the indexes in a table was invoked
    /// but the target table contains no indexes
    NoIndexesFound,
}

impl From<ParseIntError> for SqliteDiscoveryError {
    fn from(_: ParseIntError) -> Self {
        SqliteDiscoveryError::ParseIntError
    }
}

impl From<ParseFloatError> for SqliteDiscoveryError {
    fn from(_: ParseFloatError) -> Self {
        SqliteDiscoveryError::ParseFloatError
    }
}

impl From<sqlx::Error> for SqliteDiscoveryError {
    fn from(error: sqlx::Error) -> Self {
        SqliteDiscoveryError::SqlxError(error)
    }
}
