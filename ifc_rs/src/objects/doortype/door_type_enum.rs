use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::*;

/// This enumeration defines the different predefined types of an IfcDoor or
/// IfcDoorType object.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifcsharedbldgelements/lexical/ifcdoortypeenum.htm
#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum DoorTypeEnum {
    /// A standard door usually within a wall opening, as a door panel in a
    /// curtain wall, or as a "free standing" door.
    #[strum(to_string = ".DOOR.")]
    Door,

    /// A gate is a point of entry to a property usually within an opening in a
    /// fence. Or as a "free standing" gate.
    #[strum(to_string = ".GATE.")]
    Gate,

    /// A special door that lies horizonally in a slab opening. Often used for
    /// accessing cellar or attic.
    #[strum(to_string = ".TRAPDOOR.")]
    Trapdoor,

    /// User-defined door element
    #[strum(to_string = ".USERDEFINED.")]
    UserDefined,

    /// Undefined door element.
    #[strum(to_string = ".NOTDEFINED.")]
    NotDefined,
}

impl IFCParse for DoorTypeEnum {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid DoorTypeEnum")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
