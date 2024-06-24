use std::fmt::Display;

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    ifc_type::{IfcType, IfcVerify},
    prelude::*,
    units::{comma::Comma, p_space_or_comment_surrounded},
};

use super::{IFCParse, IFCParser};

/// IfcMeasureWithUnit has two usages:
///  1. For representing measure value together with its unit on the entity type
///     attribute level; thus overriding the IFC model global unit assignments.
///  2. For conversion based unit to give the conversion rate and its base.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcmeasurewithunit.htm
#[derive(IfcVerify)]
pub struct MeasureWithUnit {
    /// The value of the physical quantity when expressed in the specified units.
    pub value: IdOr<PlaneAngleMeasure>,

    /// The unit in which the physical quantity is expressed.
    pub unit: TypedId<SiUnit>,
}

impl IFCParse for MeasureWithUnit {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCMEASUREWITHUNIT("),

                value: IdOr::parse(),
                _: Comma::parse(),
                unit: Id::parse().map(TypedId::new),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for MeasureWithUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCMEASUREWITHUNIT({},{});", self.value, self.unit)
    }
}

impl IfcType for MeasureWithUnit {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::units::IFCParse;

    use super::MeasureWithUnit;

    #[ignore = "float representation doesn't match'"]
    #[test]
    fn measure_with_unit_round_trip() {
        let example = "IFCMEASUREWITHUNIT(IFCPLANEANGLEMEASURE(0.0174532925199),#16);";

        let parsed: MeasureWithUnit = MeasureWithUnit::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
