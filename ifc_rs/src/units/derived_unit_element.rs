use std::fmt::Display;

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    prelude::*,
    units::{comma::Comma, p_space_or_comment_surrounded},
};

use super::{ifc_integer::IfcInteger, IFCParse, IFCParser};

/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcderivedunitelement.htm
#[derive(IfcVerify)]
pub struct DerivedUnitElement {
    /// The fixed quantity which is used as the mathematical factor.
    pub unit: TypedId<SiUnit>,

    /// The power that is applied to the unit attribute.
    pub exponent: IfcInteger,
}

impl IFCParse for DerivedUnitElement {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCDERIVEDUNITELEMENT("),

                unit: Id::parse().map(TypedId::new),
                _: Comma::parse(),
                exponent: IfcInteger::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for DerivedUnitElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCDERIVEDUNITELEMENT({},{});", self.unit, self.exponent)
    }
}

impl IfcType for DerivedUnitElement {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::units::IFCParse;

    use super::DerivedUnitElement;

    #[test]
    fn derived_unit_element_round_trip() {
        let example = "IFCDERIVEDUNITELEMENT(#29,1);";

        let parsed: DerivedUnitElement = DerivedUnitElement::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
