use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::*;

/// This enumeration defines the available generic types for IfcSpace and
/// IfcSpaceType.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcproductextension/lexical/ifcspacetypeenum.htm
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum SpaceTypeEnum {
    /// Any space not falling into another category.
    #[strum(to_string = ".SPACE.")]
    Space,

    /// A space dedication for use as a parking spot for vehicles, including
    /// access, such as a parking aisle.
    #[strum(to_string = ".PARKING.")]
    Parking,

    /// Gross Floor Area - a specific kind of space for each building story that
    /// includes all net area and construction area (also the external envelop).
    /// Provision of such a specific space is often required by regulations.
    #[strum(to_string = ".GFA.")]
    Gfa,

    #[strum(to_string = ".INTERNAL.")]
    Internal,

    #[strum(to_string = ".EXTERNAL.")]
    External,

    /// User-defined space element.
    #[strum(to_string = ".USERDEFINED.")]
    UserDefined,

    /// Undefined space element.
    #[strum(to_string = ".NOTDEFINED.")]
    NotDefined,
}

impl IFCParse for SpaceTypeEnum {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid SpaceTypeEnum")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
