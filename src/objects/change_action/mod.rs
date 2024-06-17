use crate::parser::{p_space_or_comment, IFCParse, IFCParser};

use std::str::FromStr;
use winnow::Parser;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};

/// Enumeration identifying the type of change that might have occurred to the
/// object during the last session (e.g., unchanged, added, deleted, etc.).
/// This information is required in a partial model exchange scenario so that
/// an application or model server will know how an object might have been
/// affected by the previous application.
///
/// Note that only the first four enumerations should be used. The
/// `MODIFIEDADDED` and `MODIFIEDDELETED` are left for compatibility purposes
/// but should not be used.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcutilityresource/lexical/ifcchangeactionenum.htm
#[derive(Debug, EnumString, VariantNames, Display, Clone, Copy)]
pub enum ChangeAction {
    /// Object has not been modified. This is the default state.
    #[strum(to_string = ".NOCHANGE.")]
    NoChange,
    /// A modification to the object has been made by the user and application
    /// defined by the LastModifyingUser and LastModifyingApplication respectively.
    #[strum(to_string = ".MODIFIED.")]
    Modified,
    /// The object has been added by the user and application defined by the
    /// LastModifyingUser and LastModifyingApplication respectively.
    #[strum(to_string = ".ADDED.")]
    Added,
    /// The object has been deleted by the user and application defined by the
    /// LastModifyingUser and LastModifyingApplication respectively.
    #[strum(to_string = ".DELETED.")]
    Deleted,
    /// The object has been added and modified by the user and application
    /// defined by the LastModifyingUser and LastModifyingApplication respectively.
    #[strum(to_string = ".MODIFIEDADDED.")]
    ModifiedAdded,
    /// The object has been modified and deleted by the user and application
    /// defined by the LastModifyingUser and LastModifyingApplication respectively.
    #[strum(to_string = ".MODIFIEDDELETED.")]
    ModifiedDeleted,
}

impl IFCParse for ChangeAction {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid ChangeAction")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
