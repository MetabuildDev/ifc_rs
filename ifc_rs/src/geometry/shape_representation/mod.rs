mod deserialize;
mod repr_identifier;
mod repr_type;
mod serialize;

pub use repr_identifier::RepresentationIdentifier;
pub use repr_type::RepresentationType;

use std::fmt::Display;

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    id::{Id, IdOr},
    parser::{list::IfcList, optional::OptionalParameter},
    prelude::*,
};

pub enum ShapeItemEnum<'a> {
    MappedItem(&'a MappedItem),
    ExtrudedAreaSolid(&'a ExtrudedAreaSolid),
    Dummy(&'a Dummy),
    Other(&'a dyn IfcType),
}

impl<'a> Display for ShapeItemEnum<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShapeItemEnum::MappedItem(mapped_item) => write!(f, "{mapped_item}"),
            ShapeItemEnum::ExtrudedAreaSolid(solid) => write!(f, "{solid}"),
            ShapeItemEnum::Dummy(dummy) => write!(f, "{dummy}"),
            ShapeItemEnum::Other(ifc_type) => write!(f, "{ifc_type}"),
        }
    }
}

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
#[derive(IfcVerify)]
pub struct ShapeRepresentation {
    // All fields from IfcRepresentation https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcrepresentation.htm
    //
    /// Definition of the representation context for which the different subtypes of representation are valid.
    #[ifc_types(GeometricRepresentationSubContext, GeometricRepresentationContext)]
    pub context_of_items: Id,
    /// The optional identifier of the representation as used within a project.
    pub representation_identifier: OptionalParameter<RepresentationIdentifier>,
    /// The description of the type of a representation context. The representation type defines
    /// the type of geometry or topology used for representing the product representation. More
    /// information is given at the subtypes IfcShapeRepresentation and IfcTopologyRepresentation.
    /// The supported values for context type are to be specified by implementers agreements.
    pub representation_type: OptionalParameter<RepresentationType>,
    /// Set of geometric representation items that are defined for this representation.
    #[ifc_types(ExtrudedAreaSolid, PolyLine, MappedItem)]
    pub items: IfcList<Id>,
}

impl ShapeRepresentation {
    pub fn new(
        context: impl Into<IdOr<GeometricRepresentationSubContext>>,
        identifier: RepresentationIdentifier,
        repr_type: RepresentationType,
        ifc: &mut IFC,
    ) -> Self {
        Self {
            context_of_items: context.into().or_insert(ifc).id(),
            representation_identifier: identifier.into(),
            representation_type: repr_type.into(),
            items: IfcList::empty(),
        }
    }

    pub fn add_item<ITEM: ShapeItem>(mut self, item: ITEM, ifc: &mut IFC) -> Self {
        let item_id = ifc.data.insert_new(item);
        self.items.0.push(item_id.id());

        self
    }

    pub fn items<'a>(&'a self, ifc: &'a IFC) -> impl Iterator<Item = ShapeItemEnum<'a>> {
        self.items.iter().map(|item_id| {
            let item = ifc.data.get_untyped(*item_id);

            if let Some(mapped_item) = item.downcast_ref::<MappedItem>() {
                ShapeItemEnum::MappedItem(mapped_item)
            } else if let Some(extruded_area_solid) = item.downcast_ref::<ExtrudedAreaSolid>() {
                ShapeItemEnum::ExtrudedAreaSolid(extruded_area_solid)
            } else if let Some(dummy) = item.downcast_ref::<Dummy>() {
                ShapeItemEnum::Dummy(dummy)
            } else {
                ShapeItemEnum::Other(item)
            }
        })
    }

    pub fn items_of<'a, S: ShapeItem>(&'a self, ifc: &'a IFC) -> impl Iterator<Item = &'a S> {
        self.items
            .iter()
            .filter_map(|item_id| ifc.data.get_untyped(*item_id).downcast_ref())
    }

    pub fn parent_context<'a>(&'a self, ifc: &'a IFC) -> &'a GeometricRepresentationContext {
        let context = ifc.data.get_untyped(self.context_of_items);

        if let Some(sub_context) = context.downcast_ref::<GeometricRepresentationSubContext>() {
            ifc.data.get(sub_context.parent_context)
        } else if let Some(context) = context.downcast_ref::<GeometricRepresentationContext>() {
            context
        } else {
            unreachable!("checked by type checker");
        }
    }
}

impl IfcType for ShapeRepresentation {}
