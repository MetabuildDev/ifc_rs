use crate::geometry::profile_type::ProfileType;
use crate::parser::ifc_float::IfcFloat;
use crate::parser::optional::{IFCParse, OptionalParameter};
use crate::parser::*;

use super::RectangleProfileDef;

impl IFCParse for RectangleProfileDef {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCRECTANGLEPROFILEDEF("),

                profile_type: ProfileType::parse(),
                _: p_space_or_comment_surrounded(","),
                profile_name: OptionalParameter::parse(),
                _: p_space_or_comment_surrounded(","),
                position: OptionalParameter::parse(),
                _: p_space_or_comment_surrounded(","),
                x_dim: IfcFloat::parse(),
                _: p_space_or_comment_surrounded(","),
                y_dim: IfcFloat::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
