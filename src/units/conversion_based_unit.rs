use std::{fmt::Display, ops::Deref};

use ifc_verify_derive::IfcVerify;

use super::{
    ifc_float::IfcFloat, label::Label, shared::named_unit::NamedUnit, IFCParse, IFCParser,
};
use crate::{
    id::IdOr,
    ifc_type::{IfcType, IfcVerify},
    parser::optional::OptionalParameter,
    units::{comma::Comma, p_space_or_comment_surrounded},
    IFC,
};

/// An IfcConversionBasedUnit is used to define a unit that has a conversion rate to a base unit.
/// To identify some commonly used conversion based units, the standard designations
/// (case insensitive) for the Name attribute are indicated in Table 697.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcconversionbasedunit.htm
#[derive(IfcVerify)]
pub struct ConversionBasedUnit {
    named_unit: NamedUnit,

    /// The word, or group of words, by which the conversion based unit is referred to.
    pub name: OptionalParameter<Label>,

    /// The physical quantity from which the converted unit is derived.
    pub conversion_factor: OptionalParameter<IdOr<IfcFloat>>,
}

impl Deref for ConversionBasedUnit {
    type Target = NamedUnit;

    fn deref(&self) -> &Self::Target {
        &self.named_unit
    }
}

impl IFCParse for ConversionBasedUnit {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCCONVERSIONBASEDUNIT("),

                named_unit: NamedUnit::parse(),
                _: Comma::parse(),
                name: OptionalParameter::parse(),
                _: Comma::parse(),
                conversion_factor: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for ConversionBasedUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCCONVERSIONBASEDUNIT({},{},{});",
            self.named_unit, self.name, self.conversion_factor
        )
    }
}

impl IfcType for ConversionBasedUnit {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::ConversionBasedUnit;
    use crate::units::IFCParse;

    #[test]
    fn conversion_based_unit_round_trip() {
        let example = "IFCCONVERSIONBASEDUNIT(#52,.PLANEANGLEUNIT.,'DEGREE',#53);";

        let parsed: ConversionBasedUnit = ConversionBasedUnit::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
