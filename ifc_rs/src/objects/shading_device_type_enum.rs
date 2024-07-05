use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::*;

/// This enumeration defines the valid types of IfcShadingDevice or
/// IfcShadingDeviceType that can be predefined using the enumeration values.
///
/// https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/IfcShadingDeviceTypeEnum.htm
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum ShadingDeviceTypeEnum {
    /// A rooflike shelter of canvas or other material extending over a doorway,
    /// from the top of a window, over a deck, or similar, in order to provide
    /// protection, as from the sun.
    #[strum(to_string = ".AWNING.")]
    Awning,

    /// A blind with adjustable horizontal slats for admitting light and
    /// air while excluding direct sun and rain.
    #[strum(to_string = ".JALOUSIE.")]
    Parking,

    /// A mechanical device that limits the passage of light. Often used
    /// as a a solid or louvered movable cover for a window.
    #[strum(to_string = ".SHUTTER.")]
    Shutter,

    /// User-defined
    #[strum(to_string = ".USERDEFINED.")]
    UserDefined,

    /// Undefined
    #[strum(to_string = ".NOTDEFINED.")]
    NotDefined,
}

impl IFCParse for ShadingDeviceTypeEnum {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid ShadingDeviceTypeEnum")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
