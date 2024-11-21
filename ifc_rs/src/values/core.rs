use std::fmt::Display;

use winnow::combinator::alt;
use winnow::Parser;

use super::prelude::*;
use crate::parser::{IFCParse, IFCParser};

/// IfcValue is a select type for selecting between more specialised select types IfcSimpleValue,
/// IfcMeasureValue and IfcDerivedMeasureValue.
///
/// This can either be primitive types like bools, floats, integers or plain and flat domain
/// specific types like thermal transmittance.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcmeasureresource/lexical/ifcvalue.htm
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum IfcValue {
    Bool(BoolValue),
    Label(LabelValue),
    Real(RealValue),
    Identifier(IdentifierValue),
    ThermalTransmittance(ThermalTransmittanceValue),
}

impl IFCParse for IfcValue {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        alt((
            BoolValue::parse().map(Self::Bool),
            LabelValue::parse().map(Self::Label),
            RealValue::parse().map(Self::Real),
            IdentifierValue::parse().map(Self::Identifier),
            ThermalTransmittanceValue::parse().map(Self::ThermalTransmittance),
        ))
    }
}

impl Display for IfcValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IfcValue::Bool(v) => write!(f, "{v}"),
            IfcValue::Label(v) => write!(f, "{v}"),
            IfcValue::Real(v) => write!(f, "{v}"),
            IfcValue::Identifier(v) => write!(f, "{v}"),
            IfcValue::ThermalTransmittance(v) => write!(f, "{v}"),
        }
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::{IFCParse, IfcValue};

    #[test]
    fn ifc_value_bool_round_trip() {
        let example = "IfcBoolean(.FALSE.)";

        let value = IfcValue::parse().parse(example).unwrap();
        let str_value = value.to_string();

        assert_eq!(example, str_value);
    }

    #[test]
    fn ifc_value_label_round_trip() {
        let example = "IfcLabel('Foobar')";

        let value = IfcValue::parse().parse(example).unwrap();
        let str_value = value.to_string();

        assert_eq!(example, str_value);
    }

    #[test]
    fn ifc_value_id_round_trip() {
        let example = "IfcIdentifier('')";

        let value = IfcValue::parse().parse(example).unwrap();
        let str_value = value.to_string();

        assert_eq!(example, str_value);
    }

    #[test]
    fn ifc_value_real_round_trip() {
        let example = "IfcReal(0.12)";

        let value = IfcValue::parse().parse(example).unwrap();
        let str_value = value.to_string();

        assert_eq!(example, str_value);
    }

    #[test]
    fn ifc_value_thermal_transmittance_round_trip() {
        let example = "IFCTHERMALTRANSMITTANCEMEASURE(0.24)";

        let value = IfcValue::parse().parse(example).unwrap();
        let str_value = value.to_string();

        assert_eq!(example, str_value);

        let example = "IFCTHERMALTRANSMITTANCEMEASURE(0.333)";

        let value = IfcValue::parse().parse(example).unwrap();
        let str_value = value.to_string();

        assert_eq!(example, str_value);
    }
}
