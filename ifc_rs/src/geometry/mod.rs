use winnow::combinator::alt;

use crate::{
    parser::{IFCParse, IFCParser},
    prelude::*,
};

pub(crate) mod arbitrary_closed_profile_def;
pub(crate) mod axis;
pub(crate) mod dimension_count;
pub(crate) mod direction;
pub(crate) mod extruded_area_solid;
pub(crate) mod geometric_projection;
pub(crate) mod indexed_poly_curve;
pub(crate) mod local_placement;
pub(crate) mod non_uniform_transformations;
pub(crate) mod point;
pub(crate) mod point_list;
pub(crate) mod polyline;
pub(crate) mod prelude;
pub(crate) mod product_definition_shape;
pub(crate) mod profile_type;
pub(crate) mod rectangle_profile_def;
pub(crate) mod representation_context;
pub(crate) mod representation_subcontext;
pub(crate) mod shape_representation;
pub(crate) mod transform_base;
pub(crate) mod uniform_transformations;

pub struct Geometry;

impl Geometry {
    pub fn parse<'a>() -> impl IFCParser<'a, Box<dyn IfcType>> {
        alt((
            arbitrary_closed_profile_def::ArbitraryClosedProfileDef::parse_any(),
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
            non_uniform_transformations::CartesianTransformationOperator3DnonUniform::parse_any(),
            uniform_transformations::CartesianTransformationOperator3D::parse_any(),
        ))
    }
}
