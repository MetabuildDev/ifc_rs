use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::optional::IFCParse;
use crate::parser::*;

/// The IfcDimensionCount is restricted to have the dimensionality of either 1, 2, or 3
/// - the WR1 had been added as an addition to the STEP P42 entity dimension_count.
/// In contrary to the STEP P42 constraint, that all geometric representation items
/// within a geometric representation context are forced to have the same dimension count,
/// the IFC geometry allows mixed dimensions, particularly when defining the boundary of planar surfaces.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcgeometryresource/lexical/ifcdimensioncount.htm
#[derive(Debug, EnumString, VariantNames, Display, Clone, Copy)]
pub enum DimensionCount {
    #[strum(to_string = "1")]
    One,

    #[strum(to_string = "2")]
    Two,

    #[strum(to_string = "3")]
    Three,
}

impl IFCParse for DimensionCount {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid DimensionCount")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
