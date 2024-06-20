use winnow::combinator::alt;

use crate::{
    ifc_type::IfcType,
    parser::{IFCParse, IFCParser},
};

use self::indexed_poly_curve::IndexedPolyCurve;

pub mod arbitrary_closed_profile_def;
pub mod axis;
pub mod bounding_box;
pub mod dimension_count;
pub mod direction;
pub mod extruded_area_solid;
pub mod faceted_brep;
pub mod geometric_projection;
pub mod indexed_poly_curve;
pub mod local_placement;
pub mod point;
pub mod point_list;
pub mod polyline;
pub mod prelude;
pub mod product_definition_shape;
pub mod profile_type;
pub mod rectangle_profile_def;
pub mod representation_context;
pub mod representation_subcontext;
pub mod shape_representation;
pub mod transformations;

pub struct Geometry;

impl Geometry {
    pub fn parse<'a>() -> impl IFCParser<'a, Box<dyn IfcType>> {
        alt((
            arbitrary_closed_profile_def::ArbitraryClosedProfileDef::<IndexedPolyCurve>::parse_any(
            ),
            axis::Axis2D::parse_any(),
            axis::Axis3D::parse_any(),
            direction::Direction2D::parse_any(),
            direction::Direction3D::parse_any(),
            extruded_area_solid::ExtrudedAreaSolid::parse_any(),
            indexed_poly_curve::IndexedPolyCurve::parse_any(),
            point::Point2D::parse_any(),
            point::Point3D::parse_any(),
            point_list::PointList2D::parse_any(),
            point_list::PointList3D::parse_any(),
            polyline::PolyLine::parse_any(),
            product_definition_shape::ProductDefinitionShape::parse_any(),
            rectangle_profile_def::RectangleProfileDef::parse_any(),
            representation_context::GeometricRepresentationContext::parse_any(),
            representation_subcontext::GeometricRepresentationSubContext::parse_any(),
            shape_representation::ShapeRepresentation::parse_any(),
            local_placement::LocalPlacement::parse_any(),
            transformations::CartesianTransformationOperator3DnonUniform::parse_any(),
            bounding_box::BoundingBox::parse_any(),
            faceted_brep::FacetedBrep::parse_any(),
        ))
    }
}
