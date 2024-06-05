use comma::Comma;

use crate::id::Id;
use crate::parser::ifc_float::IfcFloat;
use crate::parser::*;

use super::ExtrudedAreaSolid;

impl IFCParse for ExtrudedAreaSolid {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCEXTRUDEDAREASOLID("),

                swept_area: Id::parse(),
                _: Comma::parse(),
                position: Id::parse(),
                _: Comma::parse(),
                extruded_direction: Id::parse(),
                _: Comma::parse(),
                depth: IfcFloat::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
