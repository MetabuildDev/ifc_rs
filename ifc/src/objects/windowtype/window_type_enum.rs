use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::*;

/// This enumeration defines the different predefined types of windows that can
/// further specify an IfcWindow or IfcWindowType.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcsharedbldgelements/lexical/ifcwindowtypeenum.htm
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum WindowTypeEnum {
    /// A standard window usually within a wall opening, as a window panel in a
    /// curtain wall, or as a "free standing" window.
    #[strum(to_string = ".WINDOW.")]
    Window,

    /// A window within a sloped building element, usually a roof slab.
    #[strum(to_string = ".SKYLIGHT.")]
    Skylight,

    /// A special window that lies horizonally in a roof slab opening.
    #[strum(to_string = ".LIGHTDOME.")]
    Lightdome,

    /// User-defined window element
    #[strum(to_string = ".USERDEFINED.")]
    UserDefined,

    /// Undefined window element.
    #[strum(to_string = ".NOTDEFINED.")]
    NotDefined,
}

impl IFCParse for WindowTypeEnum {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid WindowTypeEnum")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
