use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::*;

/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcproductextension/lexical/ifcelementcompositionenum.htm
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum CompositionTypeEnum {
    /// A group or aggregation of similar elements.
    #[strum(to_string = ".COMPLEX.")]
    Comples,

    /// An (undivided) element itself.
    #[strum(to_string = ".ELEMENT.")]
    Element,

    /// A subelement or part.
    #[strum(to_string = ".PARTIAL.")]
    Partial,
}

impl IFCParse for CompositionTypeEnum {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid CompositionTypeEnum")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
