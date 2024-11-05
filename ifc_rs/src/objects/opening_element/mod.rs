mod deserialize;
pub mod opening_element_type_enum;
mod serialize;

use std::ops::{Deref, DerefMut};

use ifc_rs_verify_derive::IfcVerify;

use super::{
    shared::{
        element::{Element, ElementBuilder},
        object::{Object, ObjectBuilder},
        product::{Product, ProductBuilder},
        root::{Root, RootBuilder},
    },
    Structure,
};
use crate::{
    id::TypedId,
    parser::{label::Label, optional::OptionalParameter},
    prelude::*,
};

/// The opening element stands for opening, recess or chase, all reflecting
/// voids. It represents a void within any element that has physical
/// manifestation. Openings can be inserted into walls, slabs, beams,
/// columns, or other elements.
///
///
/// There are two different types of opening elements:
///
/// * an opening, where the thickness of the opening is greater or equal to
///   the thickness of the element;
/// * a recess or niche, where the thickness of the recess is smaller than the
///   thickness of the element.
///
/// The attribute PredefinedType should be used to capture the differences,
///
///  * the attribute is set to OPENING for an opening or
///  * the attribute is set to RECESS for a recess or niche.
///
///  If the value for PredefinedType is omitted, or the value is set to
///  NOTDEFINED, no specific information of whether it is an opening or
///  recess shall be assumed.
///
/// An IfcOpeningElement has to be inserted into an IfcElement by using the
/// IfcRelVoidsElement relationship. It may be filled by an IfcDoor, IfcWindow,
/// or another filling element by using the relationship IfcRelFillsElements.
/// Depending on the type of the IfcShapeRepresentation of the
/// IfcOpeningElement the voiding relationship implies:
///
/// * if the IfcShapeRepresentation. = 'Body', then the Body shape
///   represntation of the opening has to be subtracted from the body shape
///   representation of the voided element - implicit Boolean difference operation.
/// * if the IfcShapeRepresentation. = 'Reference', then the Reference shape
///   representation of the opening is not subtracted, it is provided in
///   addition to the hole in the Body shape representation of the voided element.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcproductextension/lexical/ifcopeningelement.htm
#[derive(IfcVerify)]
pub struct OpeningElement {
    #[inherited]
    element: Element,

    /// Predefined generic type for an opening that is specified in an
    /// enumeration. There may be a property set given specificly for the
    /// predefined types.
    pub predefined_type: OptionalParameter<OpeningElementTypeEnum>,
}

impl OpeningElement {
    pub fn new(name: impl Into<Label>) -> Self {
        Self {
            element: Element::new(Product::new(Object::new(Root::new(name.into())))),
            predefined_type: OptionalParameter::omitted(),
        }
    }

    pub fn predefined_type(mut self, predefined_type: impl Into<OpeningElementTypeEnum>) -> Self {
        self.predefined_type = predefined_type.into().into();
        self
    }
}

impl RootBuilder for OpeningElement {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.element
    }
}

impl ObjectBuilder for OpeningElement {
    fn object_mut(&mut self) -> &mut Object {
        &mut self.element
    }
}

impl ProductBuilder for OpeningElement {
    fn product_mut(&mut self) -> &mut Product {
        &mut self.element
    }
}

impl ElementBuilder for OpeningElement {
    fn element_mut(&mut self) -> &mut Element {
        &mut self.element
    }
}

impl Deref for OpeningElement {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

impl DerefMut for OpeningElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.element
    }
}

impl IfcType for OpeningElement {}
impl Structure for OpeningElement {}

impl TransformableType for OpeningElement {
    fn shape(&self) -> Option<TypedId<ProductDefinitionShape>> {
        self.representation.custom().cloned()
    }
}
