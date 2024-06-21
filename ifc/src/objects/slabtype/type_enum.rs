use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::*;

/// This enumeration defines the available predefined types of a slab. The
/// IfcSlabTypeEnum can be used for slab occurrences, IfcSlab, and slab
/// types, IfcSlabType. A special property set definition may be provided
/// for each predefined type.
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum SlabTypeEnum {
    /// The slab is used to represent a floor slab.
    #[strum(to_string = ".FLOOR.")]
    Floor,

    /// The slab is used to represent a roof slab (either flat or sloped).
    #[strum(to_string = ".ROOF.")]
    Roof,

    /// The slab is used to represent a landing within a stair or ramp.
    #[strum(to_string = ".LANDING.")]
    Landing,

    /// The slab is used to represent a floor slab against the ground (and
    /// thereby being a part of the foundation)
    #[strum(to_string = ".BASESLAB.")]
    BaseSlab,

    /// User-defined slab element.
    #[strum(to_string = ".USERDEFINED.")]
    UserDefined,

    /// Undefined slab element.
    #[strum(to_string = ".NOTDEFINED.")]
    NotDefined,
}

impl IFCParse for SlabTypeEnum {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid SlabTypeEnum")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
