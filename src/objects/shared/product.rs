use std::{fmt::Display, ops::Deref};

use crate::{
    geometry::point::Point3D,
    id::{Id, IdOr},
    parser::{optional::OptionalParameter, p_space_or_comment_surrounded, IFCParse, IFCParser},
};

use super::object::Object;

/// The IfcProduct is an abstract representation of any object that relates
/// to a geometric or spatial context. An IfcProduct occurs at a specific
/// location in space if it has a geometric representation assigned. It can
/// be placed relatively to other products, but ultimately relative to the
/// project coordinate system. The ObjectPlacement attribute establishes
/// the coordinate system in which all points and directions used by the
/// geometric representation items under Representation are founded.
/// The Representation is provided by an IfcProductDefinitionShape being
/// either a geometric shape representation, or a topology representation
/// (with or without underlying geometry of the topological items).
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifckernel/lexical/ifcproduct.htm
pub struct Product {
    object: Object,

    /// Placement of the product in space, the placement can either be
    /// absolute (relative to the world coordinate system), relative
    /// (relative to the object placement of another product), or
    /// constraint (e.g. relative to grid axes). It is determined by
    /// the various subtypes of IfcObjectPlacement, which includes the
    /// axis placement information to determine the transformation for
    /// the object coordinate system.
    pub object_placement: OptionalParameter<IdOr<Point3D>>,

    /// Reference to the representations of the product, being either a
    /// representation (IfcProductRepresentation) or as a special case
    /// a shape representations (IfcProductDefinitionShape). The product
    /// definition shape provides for multiple geometric representations
    /// of the shape property of the object within the same object
    /// coordinate system, defined by the object placement.
    pub representation: OptionalParameter<Id>,
}

impl Deref for Product {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl IFCParse for Product {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                object: Object::parse(),
                _: p_space_or_comment_surrounded(","),
                object_placement: OptionalParameter::parse(),
                _: p_space_or_comment_surrounded(","),
                representation: OptionalParameter::parse(),
            }
        }
    }
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{}",
            self.object, self.object_placement, self.representation
        )
    }
}
