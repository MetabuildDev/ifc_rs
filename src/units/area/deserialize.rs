use winnow::combinator::alt;

use super::AreaUnit;
use crate::parser::*;
use crate::units::name::IfcUnitName;
use crate::units::optional::OptionalParse;
use crate::units::place_holder::PlaceHolder;
use crate::units::prefix::IfcPrefix;

impl AreaUnit {
    pub(crate) fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: (p_space_or_comment(), "IFCSIUNIT(",
                    p_space_or_comment(), PlaceHolder::parse(),
                    p_space_or_comment(), ",",
                    p_space_or_comment(), ".AREAUNIT.",
                    p_space_or_comment(), ",",
                    p_space_or_comment()),
                prefix: alt((
                    IfcPrefix::opt_parse().map(|v| Some(v)),
                    PlaceHolder::parse().map(|_| None)
                )),
                _: (p_space_or_comment(), ",", p_space_or_comment()),
                name: IfcUnitName::opt_parse(),
                _: (p_space_or_comment(), ");", p_space_or_comment()),
            }
        }
    }
}
