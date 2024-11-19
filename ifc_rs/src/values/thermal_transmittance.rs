use real::format_non_sci_double;
use std::fmt::Display;
use winnow::{combinator::delimited, Parser};

use crate::parser::{real::RealPrimitive, *};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ThermalTransmittanceValue(pub RealPrimitive);

impl IFCParse for ThermalTransmittanceValue {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        delimited(
            "IFCTHERMALTRANSMITTANCEMEASURE(",
            RealPrimitive::parse(),
            ")",
        )
        .map(Self)
    }
}

impl Display for ThermalTransmittanceValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCTHERMALTRANSMITTANCEMEASURE({})",
            format_non_sci_double(self.0 .0)
        )
    }
}

impl<T: Into<RealPrimitive>> From<T> for ThermalTransmittanceValue {
    fn from(value: T) -> Self {
        ThermalTransmittanceValue(value.into())
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::*;

    #[test]
    fn ifc_value_bool_round_trip() {
        let example = "IFCTHERMALTRANSMITTANCEMEASURE(0.24)";

        let value = ThermalTransmittanceValue::parse().parse(example).unwrap();
        let str_value = value.to_string();

        assert_eq!(example, str_value);
    }
}
