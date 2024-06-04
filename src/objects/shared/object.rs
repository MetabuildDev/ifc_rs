use std::{fmt::Display, ops::Deref};

use crate::parser::IFCParse;
use crate::parser::{
    label::Label, optional::OptionalParameter, p_space_or_comment_surrounded, IFCParser,
};

use super::root::Root;

/// An IfcObject is the generalization of any semantically treated
/// thing or process. Objects are things as they appear - i.e. occurrences.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifckernel/lexical/ifcobject.htm
pub struct Object {
    root: Root,

    /// The type denotes a particular type that indicates the object further.
    /// The use has to be established at the level of instantiable subtypes.
    /// In particular it holds the user defined type, if the enumeration
    /// of the attribute PredefinedType is set to USERDEFINED.
    pub object_type: OptionalParameter<Label>,
}

impl Deref for Object {
    type Target = Root;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl IFCParse for Object {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                root: Root::parse(),
                _: p_space_or_comment_surrounded(","),
                object_type: OptionalParameter::parse(),
            }
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.root, self.object_type)
    }
}
