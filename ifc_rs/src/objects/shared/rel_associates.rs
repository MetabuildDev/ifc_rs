use std::ops::DerefMut;
use std::{fmt::Display, ops::Deref};

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    id::{Id, IdOr},
    ifc_type::{IfcType, IfcVerify},
    parser::{comma::Comma, list::IfcList, IFCParse, IFCParser},
    prelude::*,
};

use super::root::Root;

/// The association relationship IfcRelAssociates refers to sources of information
/// (most notably a classification, library, document, approval, contraint, or material).
/// The information associated may reside internally or externally of the project data.
/// There is no dependency implied by the association.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcrelassociates.htm
#[derive(IfcVerify)]
pub struct RelAssociates {
    #[inherited]
    root: Root,

    /// Set of object or property definitions to which the external references or
    /// information is associated. It includes object and type objects, property set
    /// templates, property templates and property sets and contexts.
    pub related_objects: IfcList<Id>,
}

impl RelAssociates {
    pub fn new(root: Root) -> Self {
        Self {
            root,
            related_objects: IfcList::empty(),
        }
    }
}

pub trait RelAssociatesBuilder<T: IfcType>: Sized {
    fn rel_associates_mut(&mut self) -> &mut RelAssociates;

    fn relate_push(&mut self, object: impl Into<IdOr<T>>, ifc: &mut IFC) {
        self.rel_associates_mut()
            .related_objects
            .0
            .push(object.into().or_insert(ifc).id());
    }

    fn relate_obj(mut self, object: impl Into<IdOr<T>>, ifc: &mut IFC) -> Self {
        self.relate_push(object, ifc);
        self
    }
}

impl Deref for RelAssociates {
    type Target = Root;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl DerefMut for RelAssociates {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.root
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
