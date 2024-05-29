pub mod deserialize;
pub mod serialize;

use crate::{
    id::Id,
    parser::{label::Label, optional::OptionalParameter},
};

/// The IfcProductDefinitionShape defines all shape relevant information about an IfcProduct. It
/// allows for multiple geometric shape representations of the same product. The shape relevant
/// information includes:
///
/// - the shape representation including geometric representation items (for 3D solids, 2D
///   annotations, etc.) and:
///   - associated presentation information (line color, line type, surface rendering properties)
///   - assignment to presentation layers (CAD layers for visibility control)
/// - or the topological representation items for connectivity systems (vertex, edge, face
///   representations) that may include geometric representation items (vertex points, edge curves,
///   face surfaces)
pub struct ProductDefinitionShape {
    // from IfcProductRepresentation https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcproductrepresentation.htm
    //
    /// The word or group of words by which the product representation is known.
    name: OptionalParameter<Label>,
    // TODO: This should be TEXT instead
    /// The word or group of words that characterize the product representation. It can be used to
    /// add additional meaning to the name of the product representation.
    description: OptionalParameter<Label>,
    /// Contained list of representations (including shape representations). Each member defines a
    /// valid representation of a particular type within a particular representation context.
    representations: Vec<Id>,
}
