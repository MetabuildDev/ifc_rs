pub mod deserialize;
pub mod serialize;

use std::fmt::Display;

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    id::{Id, IdOr, TypedId},
    ifc_type::{IfcType, IfcVerify},
    parser::{label::Label, list::IfcList, optional::OptionalParameter},
    prelude::*,
};

pub trait ShapeItem: IfcType {}

pub enum ShapeItemEnum<'a> {
    PolyLine(IfcList<PointType<'a>>),
    ExtrudedAreaSolid(&'a ExtrudedAreaSolid),
    Dummy(&'a Dummy),
}

impl<'a> Display for ShapeItemEnum<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShapeItemEnum::PolyLine(list) => write!(f, "{list}"),
            ShapeItemEnum::ExtrudedAreaSolid(solid) => write!(f, "{solid}"),
            ShapeItemEnum::Dummy(dummy) => write!(f, "{dummy}"),
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
    pub context_of_items: TypedId<GeometricRepresentationSubContext>,
    /// The optional identifier of the representation as used within a project.
    pub representation_identifier: OptionalParameter<Label>,
    /// The description of the type of a representation context. The representation type defines
    /// the type of geometry or topology used for representing the product representation. More
    /// information is given at the subtypes IfcShapeRepresentation and IfcTopologyRepresentation.
    /// The supported values for context type are to be specified by implementers agreements.
    pub representation_type: OptionalParameter<Label>,
    /// Set of geometric representation items that are defined for this representation.
    #[ifc_types(ExtrudedAreaSolid, PolyLine, MappedItem)]
    pub items: IfcList<Id>,
}

impl ShapeRepresentation {
    pub fn new(context: impl Into<IdOr<GeometricRepresentationSubContext>>, ifc: &mut IFC) -> Self {
        Self {
            context_of_items: context.into().or_insert(ifc),
            representation_identifier: OptionalParameter::omitted(),
            representation_type: OptionalParameter::omitted(),
            items: IfcList::empty(),
        }
    }

    pub fn identifier(mut self, identifier: impl Into<Label>) -> Self {
        self.representation_identifier = identifier.into().into();
        self
    }

    pub fn repr_type(mut self, repr_type: impl Into<Label>) -> Self {
        self.representation_type = repr_type.into().into();
        self
    }

    pub fn add_item<ITEM: ShapeItem>(mut self, item: ITEM, ifc: &mut IFC) -> Self {
        let item_id = ifc.data.insert_new(item);
        self.items.0.push(item_id.id());

        self
    }

    pub fn items<'a>(&'a self, ifc: &'a IFC) -> impl Iterator<Item = ShapeItemEnum<'a>> {
        self.items.iter().map(|item_id| {
            let item = ifc.data.get_untyped(*item_id);

            if let Some(poly_line) = item.downcast_ref::<PolyLine>() {
                ShapeItemEnum::PolyLine(poly_line.points(ifc))
            } else if let Some(extruded_area_solid) = item.downcast_ref::<ExtrudedAreaSolid>() {
                ShapeItemEnum::ExtrudedAreaSolid(extruded_area_solid)
            } else if let Some(dummy) = item.downcast_ref::<Dummy>() {
                ShapeItemEnum::Dummy(dummy)
            } else {
                todo!()
            }
        })
    }
}

impl IfcType for ShapeRepresentation {}
