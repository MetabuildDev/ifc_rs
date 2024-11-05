use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    id::{Id, IdOr, TypedId},
    parser::{comma::Comma, optional::OptionalParameter, IFCParse, IFCParser},
    prelude::*,
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
#[derive(IfcVerify)]
pub struct Product {
    #[inherited]
    object: Object,

    /// Placement of the product in space, the placement can either be
    /// absolute (relative to the world coordinate system), relative
    /// (relative to the object placement of another product), or
    /// constraint (e.g. relative to grid axes). It is determined by
    /// the various subtypes of IfcObjectPlacement, which includes the
    /// axis placement information to determine the transformation for
    /// the object coordinate system.
    #[ifc_types(Axis3D, Point3D, LocalPlacement)]
    pub object_placement: OptionalParameter<Id>,

    /// Reference to the representations of the product, being either a
    /// representation (IfcProductRepresentation) or as a special case
    /// a shape representations (IfcProductDefinitionShape). The product
    /// definition shape provides for multiple geometric representations
    /// of the shape property of the object within the same object
    /// coordinate system, defined by the object placement.
    pub representation: OptionalParameter<TypedId<ProductDefinitionShape>>,
}

impl Product {
    pub fn new(object: Object) -> Self {
        Self {
            object,
            object_placement: OptionalParameter::omitted(),
            representation: OptionalParameter::omitted(),
        }
    }

    pub fn shapes<'a>(&'a self, ifc: &'a IFC) -> Vec<&'a ShapeRepresentation> {
        self.representation
            .custom()
            .map(|repr_id| {
                let repr = ifc.data.get(*repr_id);

                repr.representations
                    .0
                    .iter()
                    .map(|shape_id| ifc.data.get(*shape_id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Tries to get the extrusion direction
    pub(crate) fn direction(&self, ifc: &IFC) -> Option<Direction3D> {
        self.shapes(ifc).iter().find_map(|shape| {
            shape.items(ifc).find_map(|item| match item {
                ShapeItemEnum::ExtrudedAreaSolid(area) => {
                    let area_mappings = area.mappings(ifc);

                    Some(*area_mappings.extruded_direction)
                }
                ShapeItemEnum::MappedItem(mapped) => {
                    let ((_, shape), _) = mapped.mappings(ifc);

                    shape.items(ifc).find_map(|item| {
                        if let ShapeItemEnum::ExtrudedAreaSolid(area) = item {
                            let area_mappings = area.mappings(ifc);

                            Some(*area_mappings.extruded_direction)
                        } else {
                            None
                        }
                    })
                }

                _ => None,
            })
        })
    }

    pub(crate) fn local_placement<'a>(&self, ifc: &'a IFC) -> Option<&'a Point3D> {
        self.object_placement
            .custom()
            .and_then(|object_placement_id| {
                let local_placement = ifc
                    .data
                    .get(TypedId::<LocalPlacement>::new(*object_placement_id));

                ifc.data
                    .get_untyped(local_placement.relative_placement)
                    .downcast_ref::<Axis3D>()
                    .map(|axis| ifc.data.get(axis.location))
            })
    }
}

pub trait ProductBuilder: Sized {
    fn product_mut(&mut self) -> &mut Product;

    fn object_placement(
        mut self,
        object_placement: impl Into<IdOr<LocalPlacement>>,
        ifc: &mut IFC,
    ) -> Self {
        self.product_mut().object_placement = object_placement.into().or_insert(ifc).id().into();
        self
    }

    fn representation(
        mut self,
        representation: impl Into<IdOr<ProductDefinitionShape>>,
        ifc: &mut IFC,
    ) -> Self {
        self.product_mut().representation = representation.into().or_insert(ifc).into();
        self
    }
}

impl Deref for Product {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl DerefMut for Product {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}

impl IFCParse for Product {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                object: Object::parse(),
                _: Comma::parse(),
                object_placement: OptionalParameter::parse(),
                _: Comma::parse(),
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
