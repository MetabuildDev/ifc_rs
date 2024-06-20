use std::fmt::Display;

use ifc_verify_derive::IfcVerify;

use crate::{
    ifc_type::{IfcType, IfcVerify},
    prelude::*,
    units::{comma::Comma, p_space_or_comment_surrounded},
};

use super::{label::Label, list::IfcList, optional::OptionalParameter, IFCParse, IFCParser};

/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcderivedunit.htm
#[derive(IfcVerify)]
pub struct DerivedUnit {
    /// The group of units and their exponents that define the derived unit.
    pub elements: IfcList<TypedId<DerivedUnitElement>>,

    /// Name of the derived unit chosen from an enumeration of derived unit types for use in IFC models.
    pub unit_type: DerivedUnitEnum,

    /// Name of the derived unit chosen from an enumeration of derived unit types for use in IFC models.
    pub user_defined_type: OptionalParameter<Label>,
}

impl IFCParse for DerivedUnit {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCDERIVEDUNIT("),

                elements: IfcList::parse(),
                _: Comma::parse(),
                unit_type: DerivedUnitEnum::parse(),
                _: Comma::parse(),
                user_defined_type: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for DerivedUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCDERIVEDUNIT({},{},{});",
            self.elements, self.unit_type, self.user_defined_type
        )
    }
}

impl IfcType for DerivedUnit {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::units::IFCParse;

    use super::DerivedUnit;

    #[test]
    fn derived_unit_round_trip() {
        let example = "IFCDERIVEDUNIT((#30,#32,#34),.THERMALCONDUCTANCEUNIT.,$);";

        let parsed: DerivedUnit = DerivedUnit::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
