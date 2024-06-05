use comma::Comma;
use winnow::combinator::alt;

use super::AreaUnit;
use crate::parser::*;
use crate::units::name::IfcUnitName;
use crate::units::optional::OptionalParameter;
use crate::units::place_holder::{Inherited, Omitted};

impl IFCParse for AreaUnit {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: (p_space_or_comment_surrounded("IFCSIUNIT("),
                    alt(
                        (
                            Omitted::parse().map(drop),
                            Inherited::parse().map(drop)
                        )
                    ),
                    Comma::parse(),
                    ".AREAUNIT.",
                    Comma::parse()),
                prefix: OptionalParameter::parse(),
                _: Comma::parse(),
                name: IfcUnitName::parse(),
                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
