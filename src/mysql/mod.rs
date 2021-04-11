#[cfg(feature="def")]
#[cfg_attr(docsrs, doc(cfg(feature = "def")))]
pub mod def;

#[cfg(feature="discovery")]
#[cfg_attr(docsrs, doc(cfg(feature = "discovery")))]
pub mod discovery;

#[cfg(feature="parser")]
#[cfg_attr(docsrs, doc(cfg(feature = "parser")))]
pub mod parser;

#[cfg(feature="query")]
#[cfg_attr(docsrs, doc(cfg(feature = "query")))]
pub mod query;