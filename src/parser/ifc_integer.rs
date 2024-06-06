use std::fmt::Display;

use winnow::ascii::dec_int;
use winnow::Parser;

use crate::parser::{IFCParse, IFCParser};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct IfcInteger(pub i64);

impl IFCParse for IfcInteger {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        dec_int.map(Self)
    }
}

impl Display for IfcInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
