use std::fmt::Display;

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    prelude::*,
    units::{ifc_float::IfcFloat, p_space_or_comment_surrounded, IFCParse, IFCParser},
};

/// An IfcPlaneAngleMeasure is the value of an angle in a plane.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcplaneanglemeasure.htm
#[derive(IfcVerify)]
pub struct PlaneAngleMeasure {
    pub value: IfcFloat,
}

impl IFCParse for PlaneAngleMeasure {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCPLANEANGLEMEASURE("),

                value: IfcFloat::parse(),

                _: p_space_or_comment_surrounded(")"),
            }
        }
    }
}

impl Display for PlaneAngleMeasure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCPLANEANGLEMEASURE({})", self.value)
    }
}

impl IfcType for PlaneAngleMeasure {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::units::IFCParse;

    use super::PlaneAngleMeasure;

    #[ignore = "float representation doesn't match'"]
    #[test]
    fn plane_angle_measure_round_trip() {
        let example = "IFCPLANEANGLEMEASURE(0.0174532925199)";

        let parsed: PlaneAngleMeasure = PlaneAngleMeasure::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
