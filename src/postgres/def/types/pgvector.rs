#[cfg(feature = "with-serde")]
use serde::{Deserialize, Serialize};

/// Defines an enum for the PostgreSQL module
#[derive(Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct VectorDef {
    pub length: Option<u32>,
}

impl VectorDef {
    fn with_len(len: u32) -> Self {
        Self { length: Some(len) }
    }

    pub fn parse_str(input: &str) -> Option<Self> {
        if input == "vector" {
            return Some(Self::default());
        };

        parse_len(input).map(Self::with_len)
    }
}

fn parse_len(input: &str) -> Option<u32> {
    if let Some(inner) = input.strip_prefix("vector(")?.strip_suffix(")") {
        inner.trim().parse().ok()
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::parse_len;
    #[test]
    fn test_parse_len() {
        let test_cases = [
            ("vector", None),
            ("vector()", None),
            ("vector(1)", Some(1)),
            ("vector( 2 )", Some(2)),
        ];

        for (r#in, out) in test_cases {
            assert_eq!(parse_len(r#in), out)
        }
    }
}
