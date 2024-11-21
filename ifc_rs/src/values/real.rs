use std::fmt::Display;

use real::RealPrimitive;
use winnow::{combinator::delimited, Parser};

use crate::parser::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RealValue(pub RealPrimitive);

impl IFCParse for RealValue {
    fn parse<'a>() -> impl crate::parser::IFCParser<'a, Self>
    where
        Self: Sized,
    {
        delimited("IfcReal(", RealPrimitive::parse(), ")").map(Self)
    }
}

impl Display for RealValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IfcReal({})", self.0)
    }
}

impl<T: Into<RealPrimitive>> From<T> for RealValue {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::*;

    #[test]
    fn ifc_value_id_round_trip() {
        let example = "IfcReal(0.123)";

        let value = RealValue::parse().parse(example).unwrap();
        let str_value = value.to_string();

        assert_eq!(example, str_value);
    }
}
