//! This module handles discovery of a schema from an SQLite database.
//! Note that only the types specified by official
//! [SQLite documentation](https://www.sqlite.org/datatype3.html) are discovered.
//!
//! ### Usage
//! ```
//! SchemaDiscovery::new("sqlite://foo.db")
//!     .await
//!     .unwrap()
//!     .discover()
//!     .await
//!     .unwrap()
//!     .to_sql()
//! ```

#[cfg(feature = "def")]
#[cfg_attr(docsrs, doc(cfg(feature = "def")))]
pub mod def;

#[cfg(feature = "discovery")]
#[cfg_attr(docsrs, doc(cfg(feature = "discovery")))]
pub mod discovery;

mod error;
mod executor;
