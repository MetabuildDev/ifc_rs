use super::AngleUnit;
use crate::parser::optional::OptionalParameter;
use crate::parser::*;

impl AngleUnit {
    pub(crate) fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: (p_space_or_comment(), "IFCCONVERSIONBASEDUNIT(", p_space_or_comment()),
                dimensional_exponents_id: p_id(),
                _: (p_space_or_comment(), ",",
                    p_space_or_comment(), ".PLANEANGLEUNIT.",
                    p_space_or_comment(), ",",
                    p_space_or_comment()),
                parameter_1: OptionalParameter::opt_parse(),
                _: (p_space_or_comment(), ",", p_space_or_comment()),
                parameter_2: OptionalParameter::opt_parse(),
                _: (p_space_or_comment(), ");", p_space_or_comment()),
            }
        }
    }
}
