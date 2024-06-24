use crate::parser::{p_space_or_comment, IFCParse, IFCParser};

use std::str::FromStr;
use winnow::Parser;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};

/// Enumeration identifying the state or accessibility of the object
/// (e.g., read/write, locked, etc.). This concept was initially introduced
/// in IFC 2.0 as IfcModifiedFlag of type BINARY(3) FIXED and has been modified
/// in R2x to an enumeration. It was initially introduced as a first step
/// towards providing facilities for partial model exchange from a server as
/// requested by the IFC implementers. It is intended for use primarily by a
/// model server so that an application can identify the state of the object.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcutilityresource/lexical/ifcstateenum.htm
#[derive(Debug, EnumString, VariantNames, Display, Clone, Copy)]
pub enum AccessState {
    /// Object is in a Read-Write state. It may be modified by an application.
    #[strum(to_string = ".READWRITE.")]
    ReadWrite,
    /// Object is in a Read-Only state. It may be viewed but not modified by
    /// an application.
    #[strum(to_string = ".READONLY.")]
    ReadOnly,
    /// Object is in a Locked state. It may not be accessed by an application.
    #[strum(to_string = ".LOCKED.")]
    Locked,
    /// Object is in a Read-Write-Locked state. It may not be accessed by an
    /// application.
    #[strum(to_string = ".READWRITELOCKED.")]
    ReadWriteLocked,
    /// Object is in a Locked state. It may not be accessed by an application.
    #[strum(to_string = ".READONLYLOCKED.")]
    ReadOnlyLocked,
}

impl IFCParse for AccessState {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid AccessState")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
