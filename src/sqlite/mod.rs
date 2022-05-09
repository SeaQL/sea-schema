//! This module handles discovery of a schema from an SQLite database.
//! Note that only the types specified by official
//! [SQLite documentation](https://www.sqlite.org/datatype3.html) are discovered.

pub struct Sqlite;

#[cfg(feature = "def")]
#[cfg_attr(docsrs, doc(cfg(feature = "def")))]
pub mod def;

#[cfg(feature = "discovery")]
#[cfg_attr(docsrs, doc(cfg(feature = "discovery")))]
pub mod discovery;

mod error;
mod executor;

#[cfg(feature = "mutate")]
#[cfg_attr(docsrs, doc(cfg(feature = "mutate")))]
pub mod mutate;

#[cfg(feature = "probe")]
#[cfg_attr(docsrs, doc(cfg(feature = "probe")))]
pub mod probe;
