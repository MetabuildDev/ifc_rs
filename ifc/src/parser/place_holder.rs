use std::fmt::Display;

use winnow::Parser;

use crate::parser::IFCParse;

use super::IFCParser;

/// As mentioned in the wikipedia [DATA section](https://en.wikipedia.org/wiki/ISO_10303-21#DATA_section) there are two kinds of placeholders:
///
/// - omitted values (`$`)
/// - inherited values (`*`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Omitted;

impl IFCParse for Omitted {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        "$".map(|_| Self)
    }
}

impl Display for Omitted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$")
    }
}

/// As mentioned in the wikipedia [DATA section](https://en.wikipedia.org/wiki/ISO_10303-21#DATA_section) there are two kinds of placeholders:
///
/// - omitted values (`$`)
/// - inherited values (`*`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Inherited;

impl IFCParse for Inherited {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        "*".map(|_| Self)
    }
}

impl Display for Inherited {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "*")
    }
}
