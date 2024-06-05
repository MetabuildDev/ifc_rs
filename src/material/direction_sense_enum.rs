use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::*;

/// IfcDirectionSenseEnum is an enumeration denoting whether sense of
/// direction is positive or negative along the given axis.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcmaterialresource/lexical/ifcdirectionsenseenum.htm
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum DirectionSenseEnum {
    /// Direction defined to be positive.
    #[strum(to_string = ".POSITIVE.")]
    Positive,

    /// Direction defined to be negative.
    #[strum(to_string = ".NEGATIVE.")]
    Negative,
}

impl IFCParse for DirectionSenseEnum {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid DirectionSenseEnum")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
