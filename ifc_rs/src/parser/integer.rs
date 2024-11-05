use std::fmt::Display;

use winnow::ascii::dec_int;
use winnow::Parser;

use crate::parser::{IFCParse, IFCParser};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct IntegerPrimitive(pub i64);

impl IFCParse for IntegerPrimitive {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        dec_int.map(Self)
    }
}

impl Display for IntegerPrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for IntegerPrimitive {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
