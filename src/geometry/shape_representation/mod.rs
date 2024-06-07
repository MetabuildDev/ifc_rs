pub mod deserialize;
pub mod serialize;

use std::fmt::Display;

use crate::{
    id::Id,
    ifc_type::IfcType,
    parser::{label::Label, list::IfcList, optional::OptionalParameter},
    IFC,
};

use super::{
    extruded_area_solid::ExtrudedAreaSolid,
    point::{Point2D, Point3D, PointType},
    polyline::PolyLine,
};

pub enum ShapeItems<'a> {
    PolyLine(IfcList<PointType<'a>>),
    ExtrudedAreaSolid(&'a ExtrudedAreaSolid),
}

impl<'a> Display for ShapeItems<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShapeItems::PolyLine(list) => write!(f, "{list}"),
            ShapeItems::ExtrudedAreaSolid(solid) => write!(f, "{solid}"),
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
pub struct ShapeRepresentation {
    // All fields from IfcRepresentation https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcrepresentation.htm
    //
    /// Definition of the representation context for which the different subtypes of representation are valid.
    pub context_of_items: Id,
    /// The optional identifier of the representation as used within a project.
    pub representation_identifier: OptionalParameter<Label>,
    /// The description of the type of a representation context. The representation type defines
    /// the type of geometry or topology used for representing the product representation. More
    /// information is given at the subtypes IfcShapeRepresentation and IfcTopologyRepresentation.
    /// The supported values for context type are to be specified by implementers agreements.
    pub representation_type: OptionalParameter<Label>,
    /// Set of geometric representation items that are defined for this representation.
    pub items: IfcList<Id>,
}

impl ShapeRepresentation {
    pub fn items<'a>(&'a self, ifc: &'a IFC) -> impl Iterator<Item = ShapeItems<'a>> {
        self.items.iter().map(|item_id| {
            let item = ifc.data.get_untyped(*item_id);

            if let Some(poly_line) = item.downcast_ref::<PolyLine>() {
                ShapeItems::PolyLine(IfcList(
                    poly_line
                        .points
                        .iter()
                        .map(|point_id| {
                            let point = ifc.data.get_untyped(*point_id);

                            if let Some(point_2d) = point.downcast_ref::<Point2D>() {
                                PointType::D2(point_2d)
                            } else if let Some(point_3d) = point.downcast_ref::<Point3D>() {
                                PointType::D3(point_3d)
                            } else {
                                unreachable!()
                            }
                        })
                        .collect(),
                ))
            } else if let Some(extruded_area_solid) = item.downcast_ref::<ExtrudedAreaSolid>() {
                ShapeItems::ExtrudedAreaSolid(extruded_area_solid)
            } else {
                todo!()
            }
        })
    }
}

impl IfcType for ShapeRepresentation {}
