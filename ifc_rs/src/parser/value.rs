use std::fmt::Display;

use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::{IFCParse, IFCParser};

use super::{bool::IfcBool, label::Label};

/// IfcValue is a select type for selecting between more specialised select types IfcSimpleValue,
/// IfcMeasureValue and IfcDerivedMeasureValue.
///
/// This can either be primitive types like bools, floats, integers or plain and flat domain
/// specific types like thermal transmittance.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcmeasureresource/lexical/ifcvalue.htm
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum IfcValue {
    Bool(IfcBool),
    Label(Label),
    Identifier(Label),
}

impl IFCParse for IfcValue {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        alt((
            delimited("IfcBoolean(", IfcBool::parse().map(Self::Bool), ")"),
            delimited("IfcLabel(", Label::parse().map(Self::Label), ")"),
            delimited("IfcIdentifier(", Label::parse().map(Self::Identifier), ")"),
        ))
    }
}

impl Display for IfcValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IfcValue::Bool(v) => write!(f, "IfcBoolean({v})"),
            IfcValue::Label(v) => write!(f, "IfcLabel({v})"),
            IfcValue::Identifier(v) => write!(f, "IfcIdentifier({v})"),
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
}
