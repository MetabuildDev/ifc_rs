use std::{fmt::Display, ops::Deref};

use ifc_rs_verify_derive::IfcVerify;

use super::{
    name::IfcUnitName, optional::OptionalParameter, place_holder::Inherited, prefix::IfcPrefix,
    shared::named_unit::NamedUnit, unit_enum::IfcUnitEnum, IFCParse, IFCParser,
};
use crate::{
    ifc_type::{IfcType, IfcVerify},
    units::{comma::Comma, p_space_or_comment_surrounded},
    IFC,
};

/// The IfcSIUnit covers both standard base SI units such as meter and second,
/// and derived SI units such as Pascal, square meter and cubic meter.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcsiunit.htm
#[derive(IfcVerify)]
pub struct SiUnit {
    #[inherited]
    named_unit: NamedUnit,

    /// The SI Prefix for defining decimal multiples and submultiples of the unit.
    pub prefix: OptionalParameter<IfcPrefix>,

    /// The word, or group of words, by which the SI unit is referred to.
    pub name: IfcUnitName,
}

impl SiUnit {
    pub fn new(
        unit_type: impl Into<Option<IfcUnitEnum>>,
        prefix: impl Into<Option<IfcPrefix>>,
        name: IfcUnitName,
    ) -> Self {
        Self {
            named_unit: NamedUnit {
                dimensions: OptionalParameter::Inherited(Inherited),
                unit_type: unit_type.into().into(),
            },
            prefix: prefix.into().into(),
            name,
        }
    }
}

impl IfcType for SiUnit {}

impl Deref for SiUnit {
    type Target = NamedUnit;

    fn deref(&self) -> &Self::Target {
        &self.named_unit
    }
}

impl IFCParse for SiUnit {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCSIUNIT("),

                named_unit: NamedUnit::parse(),
                _: Comma::parse(),
                prefix: OptionalParameter::parse(),
                _: Comma::parse(),
                name: IfcUnitName::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for SiUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCSIUNIT({},{},{});",
            self.named_unit, self.prefix, self.name
        )
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::SiUnit;
    use crate::units::IFCParse;

    #[test]
    fn si_unit_round_trip() {
        let example = "IFCSIUNIT($,.LENGTHUNIT.,.MILLI.,.METRE.);";

        let parsed: SiUnit = SiUnit::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
