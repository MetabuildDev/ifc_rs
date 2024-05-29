use winnow::combinator::alt;

use super::AreaUnit;
use crate::parser::*;
use crate::units::name::IfcUnitName;
use crate::units::optional::{IFCParse, OptionalParameter};
use crate::units::place_holder::{Inherited, Omitted};

impl AreaUnit {
    pub(crate) fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: (p_space_or_comment(), "IFCSIUNIT(",
                    p_space_or_comment(), alt(
                        (
                            Omitted::parse().map(drop),
                            Inherited::parse().map(drop)
                        )
                    ),
                    p_space_or_comment(), ",",
                    p_space_or_comment(), ".AREAUNIT.",
                    p_space_or_comment(), ",",
                    p_space_or_comment()),
                prefix: OptionalParameter::parse(),
                _: (p_space_or_comment(), ",", p_space_or_comment()),
                name: IfcUnitName::parse(),
                _: (p_space_or_comment(), ");", p_space_or_comment()),
            }
        }
    }
}
