#[cfg(feature = "def")]
#[cfg_attr(docsr, doc(cfg(feature = "def")))]
pub mod def;

#[cfg(feature = "parser")]
#[cfg_attr(docsrs, doc(cfg(feature = "parser")))]
pub mod parser;

#[cfg(feature = "query")]
#[cfg_attr(docsr, doc(cfg(feature = "query")))]
pub mod query;
