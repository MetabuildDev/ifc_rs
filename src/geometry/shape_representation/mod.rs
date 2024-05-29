pub mod deserialize;
pub mod serialize;

use crate::{
    id::Id,
    parser::{label::Label, optional::OptionalParameter},
};

/// The IfcShapeRepresentation represents the concept of a particular geometric representation of a
/// product or a product component within a specific geometric representation context. The
/// inherited attribute RepresentationType is used to define the geometric model used for the shape
/// representation (e.g. 'SweptSolid', or 'Brep'), the inherited attribute RepresentationIdentifier
/// is used to denote the kind of the representation captured by the IfcShapeRepresentation (e.g.
/// 'Axis', 'Body', etc.).
///
/// Several representation identifiers for shape representation are included as predefined values
/// for RepresentationIdentifier. Table 707 indicates the defined list of values for
/// RepresentationIdentifier.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcshaperepresentation.htm
pub struct ShapeRepresentation {
    // All fields from IfcRepresentation https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcrepresentation.htm
    //
    /// Definition of the representation context for which the different subtypes of representation are valid.
    context_of_items: Id,
    /// The optional identifier of the representation as used within a project.
    representation_identifier: OptionalParameter<Label>,
    /// The description of the type of a representation context. The representation type defines
    /// the type of geometry or topology used for representing the product representation. More
    /// information is given at the subtypes IfcShapeRepresentation and IfcTopologyRepresentation.
    /// The supported values for context type are to be specified by implementers agreements.
    representation_type: OptionalParameter<Label>,
    /// Set of geometric representation items that are defined for this representation.
    items: Vec<Id>,
}
