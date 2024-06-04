use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::*;

/// The enumeration defines whether the definition of a profile shape shall be
/// geometrically resolved into a curve or into a surface.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcprofileresource/lexical/ifcprofiletypeenum.htm
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum ProfileType {
    #[strum(to_string = ".CURVE.")]
    Curve,

    #[strum(to_string = ".AREA.")]
    Area,
}

impl IFCParse for ProfileType {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid ProfileType")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
