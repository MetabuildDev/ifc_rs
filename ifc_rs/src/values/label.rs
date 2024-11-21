use std::fmt::Display;

use winnow::{combinator::delimited, Parser};

use crate::parser::{string::StringPrimitive, *};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct LabelValue(pub StringPrimitive);

impl IFCParse for LabelValue {
    fn parse<'a>() -> impl crate::parser::IFCParser<'a, Self>
    where
        Self: Sized,
    {
        delimited("IFCLABEL(", StringPrimitive::parse(), ")").map(Self)
    }
}

impl Display for LabelValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCLABEL({})", self.0)
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::*;

    #[test]
    fn ifc_value_label_round_trip() {
        let example = "IFCLABEL('Foobar')";

        let value = LabelValue::parse().parse(example).unwrap();
        let str_value = value.to_string();

        assert_eq!(example, str_value);
    }
}
