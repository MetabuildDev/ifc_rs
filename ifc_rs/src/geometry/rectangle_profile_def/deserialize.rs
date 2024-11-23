use crate::geometry::profile_type::ProfileType;
use crate::parser::comma::Comma;
use crate::parser::optional::OptionalParameter;
use crate::parser::real::RealPrimitive;
use crate::parser::*;

use super::RectangleProfileDef;

impl IFCParse for RectangleProfileDef {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            RectangleProfileDef {
                _: p_space_or_comment_surrounded("IFCRECTANGLEPROFILEDEF("),

                profile_type: ProfileType::parse(),
                _: Comma::parse(),
                profile_name: OptionalParameter::parse(),
                _: Comma::parse(),
                position: OptionalParameter::parse(),
                _: Comma::parse(),
                x_dim: RealPrimitive::parse(),
                _: Comma::parse(),
                y_dim: RealPrimitive::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
