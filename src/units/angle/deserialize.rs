use comma::Comma;

use super::AngleUnit;
use crate::id::Id;
use crate::parser::optional::OptionalParameter;
use crate::parser::*;

impl IFCParse for AngleUnit {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCCONVERSIONBASEDUNIT("),
                dimensional_exponents_id: Id::parse(),
                _: (Comma::parse(), ".PLANEANGLEUNIT.", Comma::parse()),
                parameter_1: OptionalParameter::parse(),
                _: Comma::parse(),
                parameter_2: OptionalParameter::parse(),
                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
