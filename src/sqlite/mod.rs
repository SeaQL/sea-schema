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

mod errors;
pub use errors::*;
mod columns;
pub use columns::*;
mod types;
pub use types::*;
mod tabledef;
pub use tabledef::*;
mod discovery;
pub use discovery::*;
