use std::fmt::Display;

use winnow::{
    combinator::{alt, delimited},
    Parser,
};

use crate::parser::{bool::BoolPrimitive, *};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BoolValue(pub BoolPrimitive);

impl IFCParse for BoolValue {
    fn parse<'a>() -> impl crate::parser::IFCParser<'a, Self>
    where
        Self: Sized,
    {
        delimited(
            (alt(("IFCBOOLEAN", "IfcBoolean")), "("),
            BoolPrimitive::parse(),
            ")",
        )
        .map(Self)
    }
}

impl Display for BoolValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCBOOLEAN({})", self.0)
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::*;

    #[test]
    fn ifc_value_bool_round_trip() {
        let example = "IFCBOOLEAN(.FALSE.)";

        let value = BoolValue::parse().parse(example).unwrap();
        let str_value = value.to_string();

        assert_eq!(example, str_value);
    }
}
