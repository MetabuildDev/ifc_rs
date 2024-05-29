use super::AngleUnit;
use crate::id::Id;
use crate::parser::optional::{IFCParse, OptionalParameter};
use crate::parser::*;

impl IFCParse for AngleUnit {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCCONVERSIONBASEDUNIT("),
                dimensional_exponents_id: Id::parse(),
                _: (p_space_or_comment_surrounded(","), ".PLANEANGLEUNIT.", p_space_or_comment_surrounded(",")),
                parameter_1: OptionalParameter::parse(),
                _: p_space_or_comment_surrounded(","),
                parameter_2: OptionalParameter::parse(),
                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
