use std::fmt::Display;

use winnow::{combinator::delimited, Parser};

use crate::parser::{string::StringPrimitive, *};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct IdentifierValue(pub StringPrimitive);

impl IFCParse for IdentifierValue {
    fn parse<'a>() -> impl crate::parser::IFCParser<'a, Self>
    where
        Self: Sized,
    {
        delimited("IFCIDENTIFIER(", StringPrimitive::parse(), ")").map(Self)
    }
}

impl Display for IdentifierValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCIDENTIFIER({})", self.0)
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::*;

    #[test]
    fn ifc_value_id_round_trip() {
        let example = "IFCIDENTIFIER('')";

        let value = IdentifierValue::parse().parse(example).unwrap();
        let str_value = value.to_string();

        assert_eq!(example, str_value);
    }
}
