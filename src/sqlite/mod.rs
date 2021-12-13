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

mod columns;
mod discovery;
mod errors;
mod executor;
mod schema;
mod tabledef;
mod types;

pub use columns::*;
pub use discovery::*;
pub use errors::*;
pub use executor::*;
pub use schema::*;
pub use tabledef::*;
pub use types::*;
