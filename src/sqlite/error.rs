use std::num::{ParseFloatError, ParseIntError};

use crate::sqlx_types::SqlxError;

/// This type simplifies error handling
pub type DiscoveryResult<T> = Result<T, SqliteDiscoveryError>;

/// All the errors that can be encountered when using this module
#[derive(Debug)]
pub enum SqliteDiscoveryError {
    /// An error parsing a string from the result of an SQLite query into an rust-language integer
    ParseIntError,
    /// An error parsing a string from the result of an SQLite query into an rust-language float
    ParseFloatError,
    /// The target index was not found
    IndexNotFound(String),
    /// The error as defined in [SqlxError]
    SqlxError(SqlxError),
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

impl From<SqlxError> for SqliteDiscoveryError {
    fn from(error: SqlxError) -> Self {
        SqliteDiscoveryError::SqlxError(error)
    }
}

impl std::error::Error for SqliteDiscoveryError {}

impl std::fmt::Display for SqliteDiscoveryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SqliteDiscoveryError::ParseIntError => write!(f, "Parse Integer Error"),
            SqliteDiscoveryError::ParseFloatError => write!(f, "Parse Float Error"),
            SqliteDiscoveryError::IndexNotFound(index) => write!(f, "Index Not Found: {index}"),
            SqliteDiscoveryError::SqlxError(e) => write!(f, "SQLx Error: {e:?}"),
        }
    }
}
