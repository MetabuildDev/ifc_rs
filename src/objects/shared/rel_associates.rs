use std::{fmt::Display, ops::Deref};

use crate::id::Id;
use crate::parser::comma::Comma;
use crate::parser::list::IfcList;
use crate::parser::IFCParse;
use crate::parser::IFCParser;

use super::root::Root;

/// The association relationship IfcRelAssociates refers to sources of information
/// (most notably a classification, library, document, approval, contraint, or material).
/// The information associated may reside internally or externally of the project data.
/// There is no dependency implied by the association.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcrelassociates.htm
pub struct RelAssociates {
    root: Root,

    /// Set of object or property definitions to which the external references or
    /// information is associated. It includes object and type objects, property set
    /// templates, property templates and property sets and contexts.
    pub related_objects: IfcList<Id>,
}

impl Deref for RelAssociates {
    type Target = Root;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl IFCParse for RelAssociates {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                root: Root::parse(),
                _: Comma::parse(),
                related_objects: IfcList::parse(),
            }
        }
    }
}

impl Display for RelAssociates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.root, self.related_objects)
    }
}
