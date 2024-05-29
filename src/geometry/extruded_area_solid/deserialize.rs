use crate::id::Id;
use crate::parser::ifc_float::IfcFloat;
use crate::parser::{optional::IFCParse, *};

use super::ExtrudedAreaSolid;

impl IFCParse for ExtrudedAreaSolid {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCEXTRUDEDAREASOLID("),

                swept_area: Id::parse(),
                _: p_space_or_comment_surrounded(","),
                position: Id::parse(),
                _: p_space_or_comment_surrounded(","),
                extruded_direction: Id::parse(),
                _: p_space_or_comment_surrounded(","),
                depth: IfcFloat::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
