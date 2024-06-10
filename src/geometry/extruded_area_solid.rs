use crate::ifc_type::IfcType;
use crate::parser::*;
use crate::{id::Id, parser::ifc_float::IfcFloat};
use comma::Comma;
use optional::OptionalParameter;

use std::fmt::Display;

use super::shape_representation::ShapeItem;

pub struct ExtrudedAreaSolid {
    pub swept_area: Id,
    pub position: OptionalParameter<Id>,
    pub extruded_direction: Id,
    pub depth: IfcFloat,
}

impl IFCParse for ExtrudedAreaSolid {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCEXTRUDEDAREASOLID("),

                swept_area: Id::parse(),
                _: Comma::parse(),
                position: OptionalParameter::parse(),
                _: Comma::parse(),
                extruded_direction: Id::parse(),
                _: Comma::parse(),
                depth: IfcFloat::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for ExtrudedAreaSolid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCEXTRUDEDAREASOLID({},{},{},{});",
            self.swept_area, self.position, self.extruded_direction, self.depth
        )
    }
}

impl IfcType for ExtrudedAreaSolid {}
impl ShapeItem for ExtrudedAreaSolid {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::parser::IFCParse;

    use super::ExtrudedAreaSolid;

    #[test]
    fn extruded_area_solid_round_trip() {
        let example = "IFCEXTRUDEDAREASOLID(#1457,#1460,#21,2.4384);";

        let area_unit: ExtrudedAreaSolid = ExtrudedAreaSolid::parse().parse(example).unwrap();
        let str_area_unit = area_unit.to_string();

        assert_eq!(example, str_area_unit);
    }
}
