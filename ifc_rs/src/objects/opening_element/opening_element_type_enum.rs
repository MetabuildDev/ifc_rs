use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::*;

/// This enumeration defines the basic types for opening elements.
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum OpeningElementTypeEnum {
    /// An opening as subtraction feature that cuts through the element it
    /// voids. It thereby creates a hole. An opening in addiion have a
    /// particular meaning for either providing a void for doors or windows,
    /// or an opening to permit flow of air and passing of light.
    #[strum(to_string = ".OPENING.")]
    Opening,

    /// An opening as subtraction feature that does not cut through the
    /// element it voids. It creates a niche or similar voiding pattern.
    #[strum(to_string = ".RECESS.")]
    Recess,

    /// User-defined opening element.
    #[strum(to_string = ".USERDEFINED.")]
    UserDefined,

    /// Undefined opening element.
    #[strum(to_string = ".NOTDEFINED.")]
    NotDefined,
}

impl IFCParse for OpeningElementTypeEnum {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid OpeningElementTypeEnum")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
