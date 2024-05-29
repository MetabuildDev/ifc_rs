use winnow::ascii::float;

use crate::id::Id;
use crate::parser::{optional::IFCParse, *};

use super::ExtrudedAreaSolid;

impl IFCParse for ExtrudedAreaSolid {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: (p_space_or_comment(), "IFCEXTRUDEDAREASOLID(", p_space_or_comment()),

                swept_area: Id::parse(),
                _: (p_space_or_comment(), ",", p_space_or_comment()),
                position: Id::parse(),
                _: (p_space_or_comment(), ",", p_space_or_comment()),
                extruded_direction: Id::parse(),
                _: (p_space_or_comment(), ",", p_space_or_comment()),
                depth: float,

                _: (p_space_or_comment(), ");", p_space_or_comment()),
            }
        }
    }
}
