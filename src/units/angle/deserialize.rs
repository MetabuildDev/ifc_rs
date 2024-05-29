use super::AngleUnit;
use crate::id::Id;
use crate::parser::optional::{IFCParse, OptionalParameter};
use crate::parser::*;

impl IFCParse for AngleUnit {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: (p_space_or_comment(), "IFCCONVERSIONBASEDUNIT(", p_space_or_comment()),
                dimensional_exponents_id: Id::parse(),
                _: (p_space_or_comment(), ",",
                    p_space_or_comment(), ".PLANEANGLEUNIT.",
                    p_space_or_comment(), ",",
                    p_space_or_comment()),
                parameter_1: OptionalParameter::parse(),
                _: (p_space_or_comment(), ",", p_space_or_comment()),
                parameter_2: OptionalParameter::parse(),
                _: (p_space_or_comment(), ");", p_space_or_comment()),
            }
        }
    }
}
