use std::fmt::Display;

use winnow::{combinator::delimited, Parser};

use crate::parser::{bool::BoolPrimitive, *};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BoolValue(pub BoolPrimitive);

impl IFCParse for BoolValue {
    fn parse<'a>() -> impl crate::parser::IFCParser<'a, Self>
    where
        Self: Sized,
    {
        delimited("IfcBoolean(", BoolPrimitive::parse(), ")").map(Self)
    }
}

impl Display for BoolValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IfcBoolean({})", self.0)
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::{BoolValue, IFCParse};

    #[test]
    fn ifc_value_bool_round_trip() {
        let example = "IfcBoolean(.FALSE.)";

        let value = BoolValue::parse().parse(example).unwrap();
        let str_value = value.to_string();

        assert_eq!(example, str_value);
    }
}
